use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use orbclient::{self, Renderer as OrbRenderer, Window as OrbWindow, WindowFlag};

use dces::prelude::{Entity, World};
use orbgl::prelude::{CairoRenderEngine, FramebufferSurface};
use orbgl_api::Canvas;

use crate::backend::*;

/// Implementation of the OrbClient based backend.
pub struct OrbitalBackend {
    inner: OrbWindow,
    mouse_buttons: (bool, bool, bool),
    mouse_position: Point,
    event_queue: RefCell<EventQueue>,
    messages: RefCell<BTreeMap<Entity, Vec<MessageBox>>>,
    canvas: Canvas,
}

impl OrbitalBackend {
    pub fn new(inner: OrbWindow) -> OrbitalBackend {
        let mut inner = inner;

        let surface = FramebufferSurface::new(
            inner.width(),
            inner.height(),
            inner.data_mut().as_mut_ptr() as *mut u8,
        );

        let render_engine = CairoRenderEngine::new(surface.clone());

        let canvas = Canvas::new(render_engine.clone());

        OrbitalBackend {
            inner,
            mouse_buttons: (false, false, false),
            mouse_position: Point::default(),
            event_queue: RefCell::new(EventQueue::default()),
            messages: RefCell::new(BTreeMap::new()),
            canvas,
        }
    }

    pub fn drain_events(&mut self) {
        self.inner.sync();

        for event in self.inner.events() {
            match event.to_option() {
                orbclient::EventOption::Mouse(mouse) => {
                    self.mouse_position.x = mouse.x as f64;
                    self.mouse_position.y = mouse.y as f64;
                    // self.event_queue
                    //     .borrow_mut()
                    //     .register_event(MouseMoveEvent {
                    //         position: self.mouse_position,
                    //     });
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
                        self.event_queue.borrow_mut().register_event(
                            MouseUpEvent {
                                button,
                                position: self.mouse_position,
                            },
                            0,
                        )
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
                        self.event_queue.borrow_mut().register_event(
                            MouseDownEvent {
                                button,
                                position: self.mouse_position,
                            },
                            0,
                        );
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
                        self.event_queue
                            .borrow_mut()
                            .register_event(KeyUpEvent { key }, 0);
                    } else {
                        self.event_queue
                            .borrow_mut()
                            .register_event(KeyDownEvent { key }, 0);
                    }
                }
                orbclient::EventOption::Quit(_quit_event) => {
                    self.event_queue
                        .borrow_mut()
                        .register_event(SystemEvent::Quit, 0);
                }
                orbclient::EventOption::Resize(resize_event) => {
                    self.event_queue.borrow_mut().register_event(
                        WindowEvent::Resize {
                            width: resize_event.width as f64,
                            height: resize_event.height as f64,
                        },
                        0,
                    );
                }
                _ => {}
            }
        }
    }

    pub fn render_context(&mut self) -> RenderContext<'_> {
        RenderContext {
            canvas: &mut self.canvas,
            renderer: &mut self.inner,
            event_queue: &self.event_queue,
        }
    }

    pub fn event_context(&mut self) -> EventContext<'_> {
        EventContext {
            event_queue: &self.event_queue,
        }
    }

    pub fn state_context(&mut self) -> StateContext<'_> {
        StateContext {
            event_queue: &self.event_queue,
            messages: &mut self.messages,
        }
    }
}

impl Drop for OrbitalBackend {
    fn drop(&mut self) {
        self.inner.sync();
    }
}

// impl Backend for OrbitalBackend {
    
// }

/// Implementation of the OrbClient based backend runner.
pub struct ShellRunner {
    pub world: Option<World<Tree>>,
    pub window_shell: Rc<RefCell<OrbitalBackend>>,
}

impl ShellRunner {
    pub fn run(&mut self, update: Rc<Cell<bool>>, running: Rc<Cell<bool>>) {
        loop {
            if !running.get() {
                break;
            }

            if let Some(world) = &mut self.world {
                world.run();
            }

            update.set(false);

            self.window_shell.borrow_mut().drain_events();
        }
    }
}

#[derive(Default)]
pub struct WindowBuilder {
    title: String,

    resizeable: bool,

    bounds: Bounds,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    pub fn bounds(mut self, bounds: impl Into<Bounds>) -> Self {
        self.bounds = bounds.into();
        self
    }

    pub fn build(self) -> OrbitalBackend {
        let mut flags = vec![];
        if self.resizeable {
            flags.push(WindowFlag::Resizable);
        }

        OrbitalBackend::new(
            OrbWindow::new_flags(
                self.bounds.x() as i32,
                self.bounds.y() as i32,
                self.bounds.width() as u32,
                self.bounds.height() as u32,
                &self.title,
                &flags,
            )
                .unwrap(),
        )
    }
}