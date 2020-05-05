use std::collections::BTreeMap;

use crate::{prelude::*, shell::ShellContext, utils::Point};

pub struct ContextProvider<'a> {
    render_objects: &'a mut BTreeMap<Entity, Box<dyn RenderObject>>,
    layouts: &'a mut BTreeMap<Entity, Box<dyn Layout>>,
    handler_map: &'a mut EventHandlerMap,
    states: &'a mut BTreeMap<Entity, Box<dyn State>>,
    event_queue: &'a mut EventQueue,
    mouse_position: Point,
    shell_context: &'a mut ShellContext<'a>,
}

impl<'a> ContextProvider<'a> {

    pub fn new(
        render_objects: &'a mut BTreeMap<Entity, Box<dyn RenderObject>>,
        layouts: &'a mut BTreeMap<Entity, Box<dyn Layout>>,
        handler_map: &'a mut EventHandlerMap,
        states: &'a mut BTreeMap<Entity, Box<dyn State>>,
        event_queue: &'a mut EventQueue,
        mouse_position: Point,
        shell_context: &'a mut ShellContext<'a>,
    ) -> Self {
        ContextProvider {
            render_objects,
            layouts,
            handler_map,
            states,
            event_queue,
            mouse_position,
            shell_context,
        }
    }

    pub fn render_objects(&self) -> &BTreeMap<Entity, Box<dyn RenderObject>> {
        self.render_objects
    }

    pub fn render_objects_mut(&mut self) -> &mut BTreeMap<Entity, Box<dyn RenderObject>> {
        self.render_objects
    }

    pub fn layouts(&self) -> &BTreeMap<Entity, Box<dyn Layout>> {
        self.layouts
    }

    pub fn layouts_mut(&mut self) -> &mut BTreeMap<Entity, Box<dyn Layout>> {
        self.layouts
    }

    pub fn handler_map(&self) -> &EventHandlerMap {
        self.handler_map
    }

    pub fn handler_map_mut(&mut self) -> &mut EventHandlerMap {
        self.handler_map
    }

    pub fn states(&self) -> &BTreeMap<Entity, Box<dyn State>> {
        self.states
    }

    pub fn states_mut(&mut self) -> &mut BTreeMap<Entity, Box<dyn State>> {
        self.states
    }

    pub fn event_queue(&self) -> &EventQueue {
        self.event_queue
    }

    pub fn event_queue_mut(&mut self) -> &mut EventQueue {
        self.event_queue
    }

    pub fn mouse_position(&self) -> Point {
        self.mouse_position
    }

    pub fn shell_context(&self) -> &ShellContext<'a> {
        self.shell_context
    }

    pub fn shell_context_mut(&mut self) -> &mut ShellContext<'a> {
        self.shell_context
    }
    
}
