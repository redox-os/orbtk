// use orbfont;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use orbclient::{Color, Window as OrbWindow};
use orbclient::{Mode, Renderer as OrbRenderer};

use {
    Backend, BackendRunner, EventContext, EventQueue, LayoutContext, MouseButton, MouseDownEvent,
    MouseEvent, MouseUpEvent, Rect, RenderContext, Renderer, Selector, SystemEvent, Theme, Tree,
    World,
};

pub struct OrbitalBackend {
    inner: OrbWindow,
    theme: Theme,
    mouse_buttons: (bool, bool, bool),
    mouse_position: (i32, i32),
    event_queue: RefCell<EventQueue>,
    running: bool,
}

impl Renderer for OrbWindow {
    fn render(&mut self, theme: &Theme) {
        // render window background
        let col = theme.color("background", &"window".into());
        let blub = col.data;
        let mut _color = format!("#{:x}", blub);
        _color.remove(0);
        _color.remove(0);
        self.set(theme.color("background", &"window".into()));
    }

    fn render_rectangle(
        &mut self,
        theme: &Theme,
        bounds: &Rect,
        selector: &Selector,
        offset: (i32, i32),
    ) {
        let b_r = theme.uint("border-radius", selector);

        let fill = theme.color("background", selector);

        self.rounded_rect(
            bounds.x + offset.0,
            bounds.y + offset.1,
            bounds.width,
            bounds.height,
            b_r,
            true,
            fill,
        );

        if theme.uint("border-width", selector) > 0 {
            let border_color = theme.color("border-color", selector);

            self.rounded_rect(
                bounds.x + offset.0,
                bounds.y + offset.1,
                bounds.width,
                bounds.height,
                b_r,
                false,
                border_color,
            );
        }
    }

    fn render_text(
        &mut self,
        theme: &Theme,
        text: &str,
        bounds: &Rect,
        selector: &Selector,
        offset: (i32, i32),
    ) {
        // if let Some(font) = &self.font {
        //     let line = font.render(text, 64.0);
        //     line.draw(&mut self.inner, 20, 20, Color::rgb(0, 0, 0));
        // } else {
        let rect = Rect::new(
            bounds.x + offset.0,
            bounds.y + offset.1,
            bounds.width,
            bounds.height,
        );
        let mut current_rect = Rect::new(
            bounds.x + offset.0,
            bounds.y + offset.1,
            bounds.width,
            bounds.height,
        );
        let x = rect.x;

        for c in text.chars() {
            if c == '\n' {
                current_rect.x = x;
                current_rect.y += 16;
            } else {
                if current_rect.x + 8 <= rect.x + rect.width as i32
                    && current_rect.y + 16 <= rect.y + rect.height as i32
                {
                    self.char(
                        current_rect.x,
                        current_rect.y,
                        c,
                        theme.color("color", selector),
                    );
                }
                current_rect.x += 8;
            }
        }
        // }
    }
}

impl OrbitalBackend {
    pub fn new(theme: Theme, inner: OrbWindow) -> OrbitalBackend {
        OrbitalBackend {
            inner,
            theme,
            mouse_buttons: (false, false, false),
            mouse_position: (0, 0),
            event_queue: RefCell::new(EventQueue::default()),
            running: false,
        }
    }
}

impl OrbRenderer for OrbitalBackend {
    fn width(&self) -> u32 {
        self.inner.width()
    }

    fn height(&self) -> u32 {
        self.inner.height()
    }

    fn data(&self) -> &[Color] {
        self.inner.data()
    }

    fn data_mut(&mut self) -> &mut [Color] {
        self.inner.data_mut()
    }

    fn sync(&mut self) -> bool {
        self.inner.sync()
    }

    fn mode(&self) -> &Cell<Mode> {
        &self.inner.mode()
    }

    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        // if let Some(ref font) = self.font {
        //     let mut buf = [0; 4];
        //     font.render(&c.encode_utf8(&mut buf), 16.0)
        //         .draw(&mut self.inner, x, y, color)
        // } else {
        self.inner.char(x, y, c, color);
        // }
    }
}

impl Drop for OrbitalBackend {
    fn drop(&mut self) {
        self.inner.sync();
    }
}

impl Backend for OrbitalBackend {
    fn drain_events(&mut self) {
        self.inner.sync();

        for event in self.inner.events() {
            match event.to_option() {
                orbclient::EventOption::Mouse(mouse) => {
                    self.mouse_position = (mouse.x, mouse.y);
                    self.event_queue
                        .borrow_mut()
                        .register_event(MouseEvent::Move((mouse.x, mouse.y)));
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
                        self.event_queue.borrow_mut().register_event(MouseUpEvent {
                            button,
                            position: self.mouse_position,
                        })
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
                        self.event_queue
                            .borrow_mut()
                            .register_event(MouseDownEvent {
                                button,
                                position: self.mouse_position,
                            })
                    }

                    self.mouse_buttons = (button.left, button.middle, button.right);
                }
                orbclient::EventOption::Quit(_quit_event) => {
                    self.event_queue
                        .borrow_mut()
                        .register_event(SystemEvent::Quit);
                    self.running = false;
                }
                _ => {}
            }
        }
    }

    fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    fn bounds(&mut self, bounds: &Rect) {
        self.inner.set_pos(bounds.x, bounds.y);
        self.inner.set_size(bounds.width, bounds.height);
    }

    fn render_context(&mut self) -> RenderContext {
        RenderContext {
            renderer: &mut self.inner,
            theme: &self.theme,
        }
    }

    fn layout_context(&mut self) -> LayoutContext {
        LayoutContext {
            window_size: self.size(),
            theme: &self.theme,
        }
    }

    fn event_context(&mut self) -> EventContext {
        EventContext {
            event_queue: &self.event_queue,
        }
    }
}

pub struct OrbitalBackendRunner {
    pub world: Option<World<Tree>>,
    pub backend: Rc<RefCell<OrbitalBackend>>,
}

impl BackendRunner for OrbitalBackendRunner {
    fn world(&mut self, world: World<Tree>) {
        self.world = Some(world);
    }
    fn run(&mut self) {
        self.backend.borrow_mut().running = true;

        loop {
            let running = self.backend.borrow().running;

            if !running {
                break;
            }

            if let Some(world) = &mut self.world {
                world.run();
            }

            self.backend.borrow_mut().drain_events();
        }
    }
}
