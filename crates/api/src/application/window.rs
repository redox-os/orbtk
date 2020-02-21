use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, World};

use crate::{prelude::*, shell, tree::Tree, utils::Point};

/// Represents a window. Each window has its own tree, event pipeline and shell.
#[derive(Default)]
pub struct WindowAdapter {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<EventHandlerMap>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub event_queue: EventQueue,
    pub messages: BTreeMap<Entity, Vec<MessageBox>>,
    pub root: Entity,
    pub mouse_position: Point,
}

pub struct WorldWrapper {
    pub world: World<Tree, StringComponentStore>,
}

impl shell::Updater for WorldWrapper {
    fn update(&mut self) {
        self.world.run();
    }
}

impl shell::WindowAdapter for WindowAdapter {
    fn resize(&mut self, width: f64, height: f64) {
        self.event_queue.register_event_with_strategy(
            WindowEvent::Resize { width, height },
            EventStrategy::Direct,
            self.root,
        );
    }

    fn mouse(&mut self, x: f64, y: f64) {
        self.mouse_position = Point::new(x, y);
        self.event_queue.register_event(
            MouseMoveEvent {
                x: self.mouse_position.x,
                y: self.mouse_position.y,
            },
            self.root,
        )
    }

    fn scroll(&mut self, delta_x: f64, delta_y: f64) {
        self.event_queue.register_event(
            ScrollEvent {
                delta: Point::new(delta_x, delta_y),
            },
            self.root,
        )
    }

    fn mouse_event(&mut self, event: shell::MouseEvent) {
        match event.state {
            shell::ButtonState::Up => {
                self.event_queue.register_event(
                    MouseUpEvent {
                        x: event.x,
                        y: event.y,
                        button: event.button,
                    },
                    self.root,
                );
                self.event_queue.register_event(
                    GlobalMouseUpEvent {
                        x: event.x,
                        y: event.y,
                        button: event.button,
                    },
                    self.root,
                );
            }
            shell::ButtonState::Down => self.event_queue.register_event(
                MouseDownEvent {
                    x: event.x,
                    y: event.y,
                    button: event.button,
                },
                self.root,
            ),
        }
    }

    fn mouse_position(&self) -> Point {
        self.mouse_position
    }

    fn key_event(&mut self, event: shell::KeyEvent) {
        match event.state {
            shell::ButtonState::Up => self
                .event_queue
                .register_event(KeyUpEvent { event }, self.root),
            shell::ButtonState::Down => self
                .event_queue
                .register_event(KeyDownEvent { event }, self.root),
        }
    }

    fn quit_event(&mut self) {
        self.event_queue
            .register_event(SystemEvent::Quit, self.root);
    }

    fn active(&mut self, active: bool) {
        self.event_queue.register_event_with_strategy(
            WindowEvent::ActiveChanged(active),
            EventStrategy::Direct,
            self.root,
        );
    }
}

impl Into<Box<dyn shell::WindowAdapter>> for WindowAdapter {
    fn into(self) -> Box<dyn shell::WindowAdapter> {
        Box::new(self)
    }
}
