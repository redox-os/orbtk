use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, shell::WindowShell, tree::Tree, utils::*};

/// The `EventStateSystem` pops events from the event queue and delegates the events to the corresponding event handlers of the widgets and updates the states.
pub struct EventStateSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub handlers: Rc<RefCell<EventHandlerMap>>,
    pub mouse_down_nodes: RefCell<Vec<Entity>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub registry: Rc<RefCell<Registry>>,
}

impl EventStateSystem {
    // fn process_top_down_event(
    //     &self,
    //     _event: &EventBox,
    //     _ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
    // ) {
    // }

    fn process_direct(&self, event: &EventBox) -> bool {
        if event.strategy == EventStrategy::Direct {
            if let Some(handlers) = self.handlers.borrow().get(&event.source) {
                handlers.iter().any(|handler| {
                    handler.handle_event(
                        &mut StatesContext::new(&mut *self.states.borrow_mut()),
                        &event,
                    )
                });
                return true;
            }
        }

        false
    }

    fn process_bottom_up_event(
        &self,
        mouse_position: Point,
        event: &EventBox,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
    ) -> bool {
        let mut matching_nodes = vec![];
        let mut update = false;

        let mut current_node = event.source;
        let root = ecm.entity_store().root;

        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        // global key handling
        if let Ok(event) = event.downcast_ref::<KeyDownEvent>() {
            if let Ok(global) = ecm.component_store_mut().get_mut::<Global>("global", root) {
                // Set this value on the keyboard state
                global.keyboard_state.set_key_state(event.event.key, true);
            }
        }

        if let Ok(event) = event.downcast_ref::<KeyUpEvent>() {
            if let Ok(global) = ecm.component_store_mut().get_mut::<Global>("global", root) {
                // Set this value on the keyboard state
                global.keyboard_state.set_key_state(event.event.key, false);
            }
        }

        let mut unknown_event = true;
        let mut clipped_parent = vec![];

        loop {
            let mut has_handler = false;
            if let Some(handlers) = self.handlers.borrow().get(&current_node) {
                if handlers.iter().any(|handler| handler.handles_event(event)) {
                    has_handler = true;
                }
            }

            if let Some(cp) = clipped_parent.last() {
                if ecm.entity_store().parent[&current_node] == Some(*cp) {
                    clipped_parent.push(current_node);
                } else {
                    clipped_parent.pop();
                }
            }

            // key down event
            if event.downcast_ref::<KeyDownEvent>().is_ok() {
                if let Some(focused) = ecm
                    .component_store()
                    .get::<Global>("global", root)
                    .unwrap()
                    .focused_widget
                {
                    if current_node == focused && has_handler {
                        matching_nodes.push(current_node);
                    }
                }

                unknown_event = false;
            }

            // key up event
            if event.downcast_ref::<KeyUpEvent>().is_ok() {
                if let Some(focused) = ecm
                    .component_store()
                    .get::<Global>("global", root)
                    .unwrap()
                    .focused_widget
                {
                    if current_node == focused && has_handler {
                        matching_nodes.push(current_node);
                    }
                }

                unknown_event = false;
            }

            // scroll handling
            if event.downcast_ref::<ScrollEvent>().is_ok() {
                if check_mouse_condition(
                    mouse_position,
                    &WidgetContainer::new(current_node, ecm, &theme),
                ) && has_handler
                {
                    matching_nodes.push(current_node);
                }

                unknown_event = false;
            }

            // click handling
            if let Ok(event) = event.downcast_ref::<ClickEvent>() {
                if check_mouse_condition(
                    event.position,
                    &WidgetContainer::new(current_node, ecm, &theme),
                ) {
                    let mut add = true;
                    if let Some(op) = clipped_parent.get(0) {
                        if !check_mouse_condition(
                            event.position,
                            &WidgetContainer::new(*op, ecm, &theme),
                        ) {
                            add = false;
                        }
                    }

                    if add && has_handler {
                        matching_nodes.push(current_node);
                    }
                }

                unknown_event = false;
            }

            // mouse down handling
            if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
                if check_mouse_condition(
                    Point::new(event.x, event.y),
                    &WidgetContainer::new(current_node, ecm, &theme),
                ) {
                    let mut add = true;
                    if let Some(op) = clipped_parent.get(0) {
                        // todo: improve check path if exists
                        if !check_mouse_condition(
                            Point::new(event.x, event.y),
                            &WidgetContainer::new(*op, ecm, &theme),
                        ) && has_handler
                        {
                            add = false;
                        }
                    }

                    if add {
                        matching_nodes.push(current_node);
                        self.mouse_down_nodes.borrow_mut().push(current_node);
                    }
                }

                unknown_event = false;
            }

            // mouse move handling
            if let Ok(event) = event.downcast_ref::<MouseMoveEvent>() {
                if check_mouse_condition(
                    Point::new(event.x, event.y),
                    &WidgetContainer::new(current_node, ecm, &theme),
                ) {
                    let mut add = true;
                    if let Some(op) = clipped_parent.get(0) {
                        // todo: improve check path if exists
                        if !check_mouse_condition(
                            Point::new(event.x, event.y),
                            &WidgetContainer::new(*op, ecm, &theme),
                        ) {
                            add = false;
                        }
                    }

                    if add && has_handler {
                        matching_nodes.push(current_node);
                    }
                }

                unknown_event = false;
            }

            // mouse up handling
            if event.downcast_ref::<MouseUpEvent>().is_ok() {
                if self.mouse_down_nodes.borrow().contains(&current_node) {
                    matching_nodes.push(current_node);
                    let index = self
                        .mouse_down_nodes
                        .borrow()
                        .iter()
                        .position(|x| *x == current_node)
                        .unwrap();
                    self.mouse_down_nodes.borrow_mut().remove(index);
                }

                unknown_event = false;
            }

            if unknown_event
                && *WidgetContainer::new(current_node, ecm, &theme).get::<bool>("enabled")
            {
                if has_handler {
                    matching_nodes.push(current_node);
                }
            }

            if let Ok(clip) = ecm.component_store().get::<bool>("clip", current_node) {
                if *clip {
                    clipped_parent.clear();
                    clipped_parent.push(current_node);
                }
            }

            let mut it = ecm.entity_store().start_node(current_node).into_iter();
            it.next();

            if let Some(node) = it.next() {
                current_node = node;
            } else {
                break;
            }
        }

        let mut handled = false;
        let mut disabled_parent = None;

        for node in matching_nodes.iter().rev() {
            if let Some(dp) = disabled_parent {
                if ecm.entity_store().parent[&node] == Some(dp) {
                    disabled_parent = Some(*node);
                    continue;
                } else {
                    disabled_parent = None;
                }
            }

            if let Ok(enabled) = ecm.component_store().get::<bool>("enabled", *node) {
                if !enabled {
                    disabled_parent = Some(*node);
                    continue;
                }
            }

            if let Some(handlers) = self.handlers.borrow().get(node) {
                handled = handlers.iter().any(|handler| {
                    handler.handle_event(
                        &mut StatesContext::new(&mut *self.states.borrow_mut()),
                        event,
                    )
                });

                update = true;
            }

            if handled {
                break;
            }
        }

        update
    }
}

impl System<Tree, StringComponentStore> for EventStateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
        let mut shell = self.shell.borrow_mut();
        let mut update = shell.update();

        loop {
            {
                let adapter = shell.adapter();
                let mouse_position = adapter.mouse_position;
                for event in adapter.event_queue.into_iter() {
                    if let Ok(event) = event.downcast_ref::<SystemEvent>() {
                        match event {
                            SystemEvent::Quit => {
                                shell.set_running(false);
                                return;
                            }
                        }
                    }

                    match event.strategy {
                        EventStrategy::Direct => {
                            if event.strategy == EventStrategy::Direct {
                                update = self.process_direct(&event) || update;
                            }
                        }
                        // EventStrategy::TopDown => {
                        //     self.process_top_down_event(&event, ecm);
                        // }
                        EventStrategy::BottomUp => {
                            let should_update =
                                self.process_bottom_up_event(mouse_position, &event, ecm);
                            update = update || should_update;
                        }
                    }
                }
            }

            shell.set_update(update);

            // handle states

            let root = ecm.entity_store().root;

            let theme = ecm
                .component_store()
                .get::<Theme>("theme", root)
                .unwrap()
                .clone();
            let mut current_node = root;

            loop {
                let mut skip = false;

                {
                    if !self.states.borrow().contains_key(&current_node) {
                        skip = true;
                    }

                    let mut keys = vec![];

                    if !skip {
                        let render_objects = &self.render_objects;
                        let layouts = &mut self.layouts.borrow_mut();
                        let handlers = &mut self.handlers.borrow_mut();
                        let registry = &mut self.registry.borrow_mut();
                        let new_states = &mut BTreeMap::new();

                        let mut ctx = Context::new(
                            (current_node, ecm),
                            &mut shell,
                            &theme,
                            render_objects,
                            layouts,
                            handlers,
                            &self.states,
                            new_states,
                        );

                        if let Some(state) = self.states.borrow_mut().get_mut(&current_node) {
                            state.update(registry, &mut ctx);
                        }

                        keys.append(&mut ctx.new_states_keys());

                        drop(ctx);

                        for key in keys {
                            let new_states = &mut BTreeMap::new();
                            let mut ctx = Context::new(
                                (key, ecm),
                                &mut shell,
                                &theme,
                                render_objects,
                                layouts,
                                handlers,
                                &self.states,
                                new_states,
                            );
                            if let Some(state) = self.states.borrow_mut().get_mut(&key) {
                                state.init(registry, &mut ctx);
                            }

                            drop(ctx);
                        }
                    }
                }
                let mut it = ecm.entity_store().start_node(current_node).into_iter();
                it.next();

                if let Some(node) = it.next() {
                    current_node = node;
                } else {
                    break;
                }
            }

            if shell.adapter().event_queue.is_empty() {
                break;
            }
        }
    }
}
