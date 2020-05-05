use std::collections::BTreeMap;

use dces::prelude::{Entity, World};

use crate::{prelude::*, shell, tree::Tree, utils::Point};

/// Represents a window. Each window has its own tree, event pipeline and shell.
pub struct ShellAdapter<'a> {
    pub render_objects: BTreeMap<Entity, Box<dyn RenderObject>>,
    pub layouts: BTreeMap<Entity, Box<dyn Layout>>,
    pub handlers: EventHandlerMap,
    pub states: BTreeMap<Entity, Box<dyn State>>,
    pub event_queue: EventQueue,
    pub mouse_position: Point,
    pub registry: Registry,
    pub world: World<Tree, StringComponentStore, ContextProvider)>,
}

impl<'a> ShellAdapter<'a> {
    fn root(&self) -> Entity {
        self.world
            .entity_component_manager()
            .entity_store()
            .root
            .unwrap()
    }
}

impl<'a> shell::ShellAdapter<'a> for ShellAdapter<'a> {
    fn resize(&mut self, width: f64, height: f64) {
        self.event_queue.register_event_with_strategy(
            WindowEvent::Resize { width, height },
            EventStrategy::Direct,
            self.root(),
        );
    }

    fn mouse(&mut self, x: f64, y: f64) {
        self.mouse_position = Point::new(x, y);
        self.event_queue.register_event(
            MouseMoveEvent {
                x: self.mouse_position.x,
                y: self.mouse_position.y,
            },
            self.root(),
        )
    }

    fn scroll(&mut self, delta_x: f64, delta_y: f64) {
        self.event_queue.register_event(
            ScrollEvent {
                delta: Point::new(delta_x, delta_y),
            },
            self.root(),
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
                    self.root(),
                );
                self.event_queue.register_event(
                    GlobalMouseUpEvent {
                        x: event.x,
                        y: event.y,
                        button: event.button,
                    },
                    self.root(),
                );
            }
            shell::ButtonState::Down => self.event_queue.register_event(
                MouseDownEvent {
                    x: event.x,
                    y: event.y,
                    button: event.button,
                },
                self.root(),
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
                .register_event(KeyUpEvent { event }, self.root()),
            shell::ButtonState::Down => self
                .event_queue
                .register_event(KeyDownEvent { event }, self.root()),
        }
    }

    fn quit_event(&mut self) {
        self.event_queue
            .register_event(SystemEvent::Quit, self.root());
    }

    fn active(&mut self, active: bool) {
        self.event_queue.register_event_with_strategy(
            WindowEvent::ActiveChanged(active),
            EventStrategy::Direct,
            self.root(),
        );
    }

    fn run(&mut self, shell_context: &mut shell::ShellContext<'_>) {
        let context_provider: ContextProvider<'static> = ContextProvider::new(
            &mut self.render_objects,
            &mut self.layouts,
            &mut self.handlers,
            &mut self.states,
            &mut self.event_queue,
            self.mouse_position,
        );
        {
            self.world.run_with_context(&mut (shell_context, &mut context_provider));
        }
      
    }
}

impl<'a> Into<Box<dyn shell::ShellAdapter<'a>>> for ShellAdapter<'a> {
    fn into(self) -> Box<dyn shell::ShellAdapter<'a>> {
        Box::new(self)
    }
}
