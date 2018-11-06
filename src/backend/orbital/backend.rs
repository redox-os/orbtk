use std::cell::{Cell, RefCell};
use std::rc::Rc;

use orbclient::{Color, Mode, Renderer as OrbRenderer, Window as OrbWindow};

use dces::World;

use backend::{Backend, BackendRunner, EventContext, LayoutContext, RenderContext};
use event::{
    EventQueue, Key, KeyDownEvent, KeyUpEvent, MouseButton, MouseDownEvent,
    MouseUpEvent, SystemEvent,
};
use structs::{Point, Rect};
use theme::Theme;
use tree::Tree;

pub struct OrbitalBackend {
    inner: OrbWindow,
    theme: Theme,
    mouse_buttons: (bool, bool, bool),
    mouse_position: Point,
    event_queue: RefCell<EventQueue>,
    running: bool,
}

impl OrbitalBackend {
    pub fn new(theme: Theme, inner: OrbWindow) -> OrbitalBackend {
        OrbitalBackend {
            inner,
            theme,
            mouse_buttons: (false, false, false),
            mouse_position: Point::default(),
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
                    self.mouse_position.x = mouse.x;
                    self.mouse_position.y = mouse.y;
                    // self.event_queue
                    //     .borrow_mut()
                    //     .register_event(MouseMouveEvent {
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
                        self.event_queue
                            .borrow_mut()
                            .register_event(KeyUpEvent { key });
                    } else {
                        self.event_queue
                            .borrow_mut()
                            .register_event(KeyDownEvent { key });
                    }
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
    fn run(&mut self, update: Rc<Cell<bool>>) {
        self.backend.borrow_mut().running = true;

        loop {
            let running = self.backend.borrow().running;

            if !running {
                break;
            }

            if let Some(world) = &mut self.world {
                world.run();
            }

            update.set(false);

            self.backend.borrow_mut().drain_events();
        }
    }
}
