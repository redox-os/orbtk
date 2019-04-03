use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time;

use orbclient::{self, Color, Mode, Renderer as OrbRenderer, Window as OrbWindow};

use dces::prelude::{Entity, World};
use orbgl::prelude::{CairoRenderEngine, FramebufferSurface};
use orbgl_api::Canvas;

use crate::application::Tree;
use crate::backend::{
    Backend, BackendRunner, EventContext, InitContext, LayoutContext, RenderContext, StateContext,
};
use crate::event::{
    EventQueue, Key, KeyDownEvent, KeyUpEvent, MouseButton, MouseDownEvent, MouseUpEvent,
    SystemEvent, WindowEvent,
};
use crate::properties::Bounds;
use crate::structs::{Point, Position, Size};
use crate::theme::Theme;
use crate::widgets::MessageBox;

/// Implementation of the OrbClient based backend.
pub struct OrbitalBackend {
    inner: OrbWindow,
    theme: Theme,
    mouse_buttons: (bool, bool, bool),
    mouse_position: Point,
    event_queue: RefCell<EventQueue>,
    messages: RefCell<BTreeMap<Entity, Vec<MessageBox>>>,
    canvas: Canvas,
}

impl OrbitalBackend {
    pub fn new(theme: Theme, inner: OrbWindow) -> OrbitalBackend {
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
            theme,
            mouse_buttons: (false, false, false),
            mouse_position: Point::default(),
            event_queue: RefCell::new(EventQueue::default()),
            messages: RefCell::new(BTreeMap::new()),
            canvas,
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

    fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    fn bounds(&mut self, bounds: &Bounds) {
        self.inner.set_pos(bounds.x() as i32, bounds.y() as i32);
        self.inner
            .set_size(bounds.width() as u32, bounds.height() as u32);
    }

    fn init_context(&mut self) -> InitContext<'_> {
        InitContext { theme: &self.theme }
    }

    fn render_context(&mut self) -> RenderContext<'_> {
        RenderContext {
            canvas: &mut self.canvas,
            renderer: &mut self.inner,
            theme: &self.theme,
            event_queue: &self.event_queue,
        }
    }

    fn layout_context(&mut self) -> LayoutContext<'_> {
        LayoutContext {
            window_size: self.size(),
            theme: &self.theme,
        }
    }

    fn event_context(&mut self) -> EventContext<'_> {
        EventContext {
            event_queue: &self.event_queue,
        }
    }

    fn state_context(&mut self) -> StateContext<'_> {
        StateContext {
            theme: &self.theme,
            event_queue: &self.event_queue,
            messages: &mut self.messages,
        }
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
    fn run(&mut self, update: Rc<Cell<bool>>, running: Rc<Cell<bool>>) {
        let target_fps = 60.0;
        let target_fps_nanos = (1. / target_fps as f32) * 1_000_000_000.;
        let mut last_tick_time = time::Instant::now();

        loop {
            let sys_time = time::SystemTime::now();
            if !running.get() {
                break;
            }

            let time = last_tick_time.elapsed();
            let total_nanos = time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64;
            let elapsed = target_fps_nanos - (total_nanos as f32);

            if elapsed <= 0. {
                if let Some(world) = &mut self.world {
                    world.run();
                }

                last_tick_time = time::Instant::now();
                update.set(false);
            }

            self.backend.borrow_mut().drain_events();
        }
    }
}
