use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, World};

use crate::prelude::*;

use crate::backend;

/// Represents a window. Each window has its own tree, event pipeline and backend.
#[derive(Default)]
pub struct WindowAdapter {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub event_queue: EventQueue,
    pub messages: BTreeMap<Entity, Vec<MessageBox>>,
    pub root: Entity,
}

// todo: remove all render_objects etc. from system, handle all with window adapter
// todo: remove rc refcell..
// impl WindowAdapter {
//     pub fn context_provider(&mut self) -> &mut ContextProvider {
//         &mut self.context_provider
//     }
// }

pub struct WorldWrapper {
    pub world: World<Tree>,
}

impl backend::Updater for WorldWrapper {
    fn update(&mut self) {
        self.world.run();
    }
}

impl backend::WindowAdapter for WindowAdapter {
    fn update(&mut self) {}

    fn resize(&mut self, width: f64, height: f64) {
        self.event_queue
            .register_event(WindowEvent::Resize { width, height }, self.root);
    }

    fn mouse(&mut self, x: f64, y: f64) {
        self.event_queue.register_event(MouseMoveEvent {
            position: Point::new(x, y)
        }, self.root)
    }

    fn mouse_event(&mut self, event: backend::MouseEvent) {
        match event.state {
            backend::ButtonState::Up => self.event_queue.register_event(
                MouseUpEvent {
                    x: event.x,
                    y: event.y,
                    button: event.button,
                },
                self.root,
            ),
            backend::ButtonState::Down => self.event_queue.register_event(
                MouseDownEvent {
                    x: event.x,
                    y: event.y,
                    button: event.button,
                },
                self.root,
            ),
        }
    }

    fn key_event(&mut self, event: backend::KeyEvent) {
         match event.state {
            backend::ButtonState::Up => self.event_queue.register_event(
                KeyUpEvent {
                    key: event.key
                },
                self.root,
            ),
            backend::ButtonState::Down => self.event_queue.register_event(
                 KeyDownEvent {
                    key: event.key
                },
                self.root,
            ),
        }
    }

    fn quite_event(&mut self) {
        self.event_queue
            .register_event(SystemEvent::Quit, self.root);
    }
}

impl Into<Box<backend::WindowAdapter>> for WindowAdapter {
    fn into(self) -> Box<backend::WindowAdapter> {
        Box::new(self)
    }
}

#[derive(Default, Debug)]
pub struct ContextProvider {}
