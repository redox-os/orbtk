use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use orbclient::{self, Color, Mode, Renderer as OrbRenderer, Window as OrbWindow};
use orbfont::Font;
use orbgl::Canvas;
use orbimage::Image;

use dces::World;

use crate::{
    application::Tree,
    core::{
        Backend, BackendRunner, EventContext, LayoutContext, OrbContext, RenderContext,
        RenderContext2D, Shape2D, StateContext,
    },
    event::{
        EventQueue, Key, KeyDownEvent, KeyUpEvent, MouseButton, MouseDownEvent, MouseUpEvent,
        SystemEvent,
    },
    properties::{Bounds, Point},
    theme::Theme,
};

/// Implemenation of the OrbClient based backend.
pub struct OrbitalBackend {
    context: OrbContext,
    mouse_buttons: (bool, bool, bool),
    mouse_position: Point,
    event_queue: RefCell<EventQueue>,
    running: bool,
}

impl OrbitalBackend {
    pub fn new(theme: Theme, inner: OrbWindow) -> OrbitalBackend {
        let canvas = Canvas::new(inner.width() as f32, inner.height() as f32);

        OrbitalBackend {
            context: OrbContext {
                orbclient_context: inner,
                orbgl_context: canvas,
                theme: theme,
                image_cache: HashMap::new(),
                fonts: HashMap::new(),
                fill_color: Color { data: 0 },
                stroke_color: Color { data: 0 },
                gradient: vec![],
                position: (0.0, 0.0),
            },
            mouse_buttons: (false, false, false),
            mouse_position: Point::default(),
            event_queue: RefCell::new(EventQueue::default()),
            running: false,
        }
    }
}

impl OrbRenderer for OrbitalBackend {
    fn width(&self) -> u32 {
        self.context.orbclient_context.width()
    }

    fn height(&self) -> u32 {
        self.context.orbclient_context.height()
    }

    fn data(&self) -> &[Color] {
        self.context.orbclient_context.data()
    }

    fn data_mut(&mut self) -> &mut [Color] {
        self.context.orbclient_context.data_mut()
    }

    fn sync(&mut self) -> bool {
        self.context.orbclient_context.sync()
    }

    fn mode(&self) -> &Cell<Mode> {
        &self.context.orbclient_context.mode()
    }

    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        // if let Some(ref font) = self.font {
        //     let mut buf = [0; 4];
        //     font.render(&c.encode_utf8(&mut buf), 16.0)
        //         .draw(&mut self.context.orbclient_context, x, y, color)
        // } else {
        self.context.orbclient_context.char(x, y, c, color);
        // }
    }
}

impl Drop for OrbitalBackend {
    fn drop(&mut self) {
        self.context.orbclient_context.sync();
    }
}

impl Backend for OrbitalBackend {
    fn drain_events(&mut self) {
        // self.context.orbclient_context.sync();

        for event in self.context.orbclient_context.events() {
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
                    self.running = false;
                }
                _ => {}
            }
        }
    }

    fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    fn bounds(&mut self, bounds: &Bounds) {
        self.context.orbclient_context.set_pos(bounds.x, bounds.y);
        self.context
            .orbclient_context
            .set_size(bounds.width, bounds.height);
    }

    fn render_context(&mut self) -> &mut RenderContext2D {
        self.context.orbclient_context.set(Color { data: 0 });
        &mut self.context
    }

    fn layout_context(&mut self) -> LayoutContext<'_> {
        LayoutContext {
            window_size: self.size(),
            theme: &self.context.theme,
        }
    }

    fn event_context(&mut self) -> EventContext<'_> {
        EventContext {
            event_queue: &self.event_queue,
        }
    }

    fn state_context(&mut self) -> StateContext<'_> {
        StateContext {
            theme: &self.context.theme,
        }
    }

    fn flip(&mut self) -> bool {
        self.sync()
    }
}

/// Implementation of the OrbClient based backend runner.
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
