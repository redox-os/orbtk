use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{css_engine::*, prelude::*, shell::WindowShell, tree::Tree, utils::*};

/// The `EventStateSystem` pops events from the event queue and delegates the events to the corresponding event handlers of the widgets and updates the states.
pub struct EventStateSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub handlers: EventHandlerMap,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub mouse_down_nodes: RefCell<Vec<Entity>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
}

impl EventStateSystem {
    fn process_top_down_event(
        &self,
        _event: &EventBox,
        _ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
    ) {
    }

    fn process_bottom_up_event(
        &self,
        mouse_position: Point,
        event: &EventBox,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
    ) {
        let mut matching_nodes = vec![];

        let mut current_node = event.source;
        let root = ecm.entity_store().root;

        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        // resize
        if let Ok(WindowEvent::Resize { width, height }) = event.downcast_ref::<WindowEvent>() {
            // update window size
            if let Ok(bounds) = ecm
                .component_store_mut()
                .get_mut::<Rectangle>("bounds", root)
            {
                bounds.set_width(*width);
                bounds.set_height(*height);
            }

            if let Ok(constraint) = ecm
                .component_store_mut()
                .get_mut::<Constraint>("constraint", root)
            {
                constraint.set_width(*width);
                constraint.set_height(*height);
            }

            self.update.set(true);
        }

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
                    if current_node == focused {
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
                    if current_node == focused {
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
                ) {
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

                    if add {
                        matching_nodes.push(current_node);
                        self.mouse_down_nodes.borrow_mut().push(current_node);
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
                        ) {
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
                if let Some(handlers) = self.handlers.borrow().get(&current_node) {
                    if handlers.iter().any(|handler| handler.handles_event(&event)) {
                        matching_nodes.push(current_node);
                    }
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
                handled = handlers.iter().any(|handler| handler.handle_event(event));

                self.update.set(true);
            }

            if handled {
                break;
            }
        }
    }
}

impl System<Tree, StringComponentStore> for EventStateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree, StringComponentStore>) {
        let mut shell = self.shell.borrow_mut();

        loop {
            {
                let adapter = shell.adapter();
                let mouse_position = adapter.mouse_position;
                for event in adapter.event_queue.into_iter() {
                    if let Ok(event) = event.downcast_ref::<SystemEvent>() {
                        match event {
                            SystemEvent::Quit => {
                                self.running.set(false);
                                return;
                            }
                        }
                    }

                    match event.strategy {
                        EventStrategy::TopDown => {
                            self.process_top_down_event(&event, ecm);
                        }
                        EventStrategy::BottomUp => {
                            self.process_bottom_up_event(mouse_position, &event, ecm);
                        }
                        _ => {}
                    }
                }
            }

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
                    let mut context = Context::new(
                        current_node,
                        ecm,
                        &mut shell,
                        &theme,
                        self.render_objects.clone(),
                        self.layouts.clone(),
                        self.handlers.clone(),
                        self.states.clone(),
                    );

                    if !self.states.borrow().contains_key(&current_node) {
                        skip = true;
                    }

                    if !skip {
                        if let Some(state) = self.states.borrow().get(&current_node) {
                            state.update(&mut context);
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
