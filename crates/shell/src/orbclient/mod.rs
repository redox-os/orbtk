use std::{collections::HashMap, sync::Arc, cell::{Cell, RefCell}, rc::Rc};

use orbclient::{Color, Window, WindowFlag, Renderer};
use orbfont::Font;
use orbgl_api::Canvas;
use orbgl::prelude::{CairoRenderEngine, FramebufferSurface};

use orbtk_utils::{Point, Rect, Position, Size};

use crate::{window, obsolete, prelude::*};

pub struct WindowShell<A> where A: WindowAdapter {
    pub inner: Window,
    mouse_buttons: (bool, bool, bool),
    mouse_position: Point,
    pub canvas: Canvas,
    adapter: A,
}

impl<A> WindowShell<A> where A: WindowAdapter {
    pub fn new(inner: Window, adapter: A) -> WindowShell<A> {
        let mut inner = inner;

        let surface = FramebufferSurface::new(
            inner.width(),
            inner.height(),
            inner.data_mut().as_mut_ptr() as *mut u8,
        );

        let render_engine = CairoRenderEngine::new(surface.clone());

        let canvas = Canvas::new(render_engine.clone());

        WindowShell {
            inner,
            mouse_buttons: (false, false, false),
            mouse_position: Point::default(),
            canvas,
            adapter,
        }
    }

    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    pub fn drain_events(&mut self) {
        self.inner.sync();

        for event in self.inner.events() {
            match event.to_option() {
                orbclient::EventOption::Mouse(event) => {
                    self.mouse_position.x = event.x as f64;
                    self.mouse_position.y = event.y as f64;
                    self.adapter.mouse(event.x as f64, event.y as f64);
                }
                orbclient::EventOption::Button(button) => {
                    if !button.left && !button.middle && !button.right {
                        let button = {
                            if self.mouse_buttons.0 {
                                MouseButton::Left
                            } else if self.mouse_buttons.1 {
                                MouseButton::Middle
                            } else {
                                MouseButton::Right
                            }
                        };

                        self.adapter.mouse_event(MouseEvent {
                            x: self.mouse_position.x,
                            y: self.mouse_position.y,
                            button,
                            state: ButtonState::Up
                        });
                    } else {
                        let button = {
                            if button.left {
                                MouseButton::Left
                            } else if button.middle {
                                MouseButton::Middle
                            } else {
                                MouseButton::Right
                            }
                        };

                        self.adapter.mouse_event(MouseEvent {
                            x: self.mouse_position.x,
                            y: self.mouse_position.y,
                            button,
                            state: ButtonState::Down
                        });
                    }

                    self.mouse_buttons = (button.left, button.middle, button.right);
                }
                orbclient::EventOption::Key(key_event) => {
                    let key = {
                        match key_event.scancode {
                            orbclient::K_BKSP => Key::Backspace,
                            orbclient::K_UP => Key::Up,
                            orbclient::K_DOWN => Key::Down,
                            orbclient::K_LEFT => Key::Left,
                            orbclient::K_RIGHT => Key::Right,
                            _ => match key_event.character {
                                '\n' => Key::Enter,
                                _ => Key::from(key_event.character),
                            },
                        }
                    };

                    if key_event.pressed {
                        self.adapter.key_event(KeyEvent { key, state: ButtonState::Up} );
                    } else {
                        self.adapter.key_event(KeyEvent { key, state: ButtonState::Down} );
                    }
                }
                orbclient::EventOption::Quit(_quit_event) => {
                    self.adapter.quite_event();
                }
                orbclient::EventOption::Resize(event) => {
                    self.adapter.resize(event.width as f64, event.height as f64);
                }
                _ => {}
            }
        }
    }
}

impl<A> Drop for WindowShell<A> where A: WindowAdapter {
    fn drop(&mut self) {
        self.inner.sync();
    }
}

/// Implementation of the OrbClient based backend runner.
pub struct ShellRunner<A> where A: WindowAdapter {
    pub window_shell: Rc<RefCell<WindowShell<A>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub updater: Box<Updater>,
}

impl<A> window::ShellRunner for ShellRunner<A> where A: WindowAdapter {
    fn run(&mut self) {
        loop {
            if !self.running.get() {
                break;
            }

            self.updater.update();

            self.update.set(false);

            self.window_shell.borrow_mut().drain_events();
        }
    }
}

pub struct WindowBuilder<A> where A: WindowAdapter {
    title: String,

    resizeable: bool,

    bounds: Rect,

    adapter: A,
}

impl<A> WindowBuilder<A> where A: WindowAdapter {
    pub fn new(adapter: A) -> Self {
        WindowBuilder {
            adapter,
            title: String::default(),
            resizeable: false,
            bounds: Rect::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    pub fn bounds(mut self, bounds: impl Into<Rect>) -> Self {
        self.bounds = bounds.into();
        self
    }

    pub fn build(self) -> WindowShell<A> {
        let mut flags = vec![];
        if self.resizeable {
            flags.push(WindowFlag::Resizable);
        }

        WindowShell::new(
            Window::new_flags(
                self.bounds.x as i32,
                self.bounds.y as i32,
                self.bounds.width as u32,
                self.bounds.height as u32,
                &self.title,
                &flags,
            ).unwrap(),
            self.adapter,
        )
    }
}

// --- obsolete will be removed after OrbGL supports text rendering ---

pub struct OrbFontMeasure;

impl FontMeasure for OrbFontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32) {
        if font_size == 0 {
            return (0, 0);
        }
        let text = font.render(text, font_size as f32);
        (text.width(), text.height())
    }
}

lazy_static! {
    pub static ref FONT_MEASURE: Arc<OrbFontMeasure> = { Arc::new(OrbFontMeasure) };
}

pub struct OrbFontRenderer {
    pub fonts: HashMap<&'static str, Font>,
}

impl OrbFontRenderer {
    fn render(
        &self,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        global_position: &Point,
        renderer: &mut Window,
        font_size: f32,
        color: Color,
        font: &Font,
    ) {
        if font_size > 0.0 {
            let line = font.render(text, font_size);
            line.draw_clipped(
                renderer,
                (global_position.x + bounds.x) as i32,
                (global_position.y + bounds.y) as i32,
                global_position.x as i32,
                parent_bounds.width as u32,
                color,
            );
        }
    }
}

// lazy_static! {
//     pub static ref FONT_RENDERER: Arc<OrbFontRenderer> = {
//         let mut fonts = HashMap::new();

//         if let Ok(font) = Font::from_data(fonts::ROBOTO_REGULAR_FONT.to_vec().into_boxed_slice()) {
//             fonts.insert("Roboto Regular", font);
//         }

//         if let Ok(font) = Font::from_data(fonts::MATERIAL_ICONS_REGULAR_FONT.to_vec().into_boxed_slice()) {
//             fonts.insert("Material Icons Regular", font);
//         }

//         Arc::new(OrbFontRenderer { fonts })
//     };
// }

// impl obsolete::Renderer for Window {
//     fn render_text(
//         &mut self,
//         text: &str,
//         bounds: &Rect,
//         parent_bounds: &Rect,
//         global_position: &Point,
//         font_size: u32,
//         color: Color,
//         font: &Font,
//     ) {
//         let alpha = (color.data >> 24) & 0xFF;

//         if alpha == 0 {
//             return;
//         }

//         FONT_RENDERER.render(
//             text,
//             bounds,
//             parent_bounds,
//             global_position,
//             self,
//             font_size as f32,
//             color,
//             font,
//         );
//     }
// }

// --- obsolete will be removed after OrbGL supports text rendering ---