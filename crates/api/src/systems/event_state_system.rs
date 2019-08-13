use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{prelude::*, shell::WindowShell, tree::Tree, utils::*};

/// The `EventStateSystem` pops events from the event queue and delegates the events to the corresponding event handlers of the widgets and updates the states.
pub struct EventStateSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub mouse_down_nodes: RefCell<Vec<Entity>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
}

impl EventStateSystem {
    fn process_top_down_event(&self, _event: &EventBox, _ecm: &mut EntityComponentManager<Tree>) {}

    fn process_bottom_up_event(&self, event: &EventBox, ecm: &mut EntityComponentManager<Tree>) {
        let mut matching_nodes = vec![];

        let mut current_node = event.source;

        loop {
            let widget = WidgetContainer::new(current_node, ecm);

            // click handling
            if let Ok(event) = event.downcast_ref::<ClickEvent>() {
                if check_mouse_condition(event.position, &widget) {
                    matching_nodes.push(current_node);
                    self.mouse_down_nodes.borrow_mut().push(current_node);
                }
            }

            // mouse down handling
            if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
                if check_mouse_condition(Point::new(event.x, event.y), &widget) {
                    matching_nodes.push(current_node);
                    self.mouse_down_nodes.borrow_mut().push(current_node);
                }
            }

            // mouse up handling
            if let Ok(_) = event.downcast_ref::<MouseUpEvent>() {
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

            if let Ok(enabled) = ecm.component_store().borrow_component::<Enabled>(*node) {
                if !enabled.0 {
                    disabled_parent = Some(*node);
                    continue;
                }
            }

            if let Some(handlers) = self.handlers.borrow().get(node) {
                for handler in handlers {
                    handled = handler.handle_event(event);

                    if handled {
                        break;
                    }
                }

                self.update.set(true);
            }

            if handled {
                break;
            }
        }

        let root = ecm.entity_store().root;

        // KeyDown handling
        if let Ok(event) = event.downcast_ref::<KeyDownEvent>() {
            if let Ok(global) = ecm
                .component_store_mut()
                .borrow_mut_component::<Global>(root)
            {
                // Set this value on the keyboard state
                global.keyboard_state.set_key_state(event.event.key, true);
            }
        }

        // KeyUp handling
        if let Ok(event) = event.downcast_ref::<KeyUpEvent>() {
            if let Ok(global) = ecm
                .component_store_mut()
                .borrow_mut_component::<Global>(root)
            {
                // Set this value on the keyboard state
                global.keyboard_state.set_key_state(event.event.key, false);
            }
        }
    }

    fn has_default_flags(&self, widget: &WidgetContainer<'_>) -> bool {
        return widget.has::<Enabled>();
    }

    // Used to updates default states like Pressed, Focused and Enabled.
    fn update_default_states(&self, widget: &mut WidgetContainer<'_>) {
        let mut enabled = (false, false);
        if let Some(en) = widget.try_get::<Enabled>() {
            enabled = (true, en.0);
        }

        if enabled.0 {
            self.update_default_state(!enabled.1, "disabled", widget);
        }
    }

    // Updates the pseudo class of a widget by the given state.
    fn update_default_state(
        &self,
        state: bool,
        pseudo_class: &str,
        widget: &mut WidgetContainer<'_>,
    ) {
        if state {
            add_selector_to_widget(pseudo_class, widget)
        } else {
            remove_selector_from_widget(pseudo_class, widget);
        }
    }
}

impl System<Tree> for EventStateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>) {
        let mut shell = self.shell.borrow_mut();

        // let mut new_events = vec![];
        let root = ecm.entity_store().root;

        loop {
            {
                let adapter = shell.adapter();
                for event in adapter.event_queue.into_iter() {
                    if let Ok(event) = event.downcast_ref::<WindowEvent>() {
                        match event {
                            WindowEvent::Resize { width, height } => {
                                // update window size
                                if let Ok(bounds) = ecm
                                    .component_store_mut()
                                    .borrow_mut_component::<Bounds>(root)
                                {
                                    bounds.set_width(*width);
                                    bounds.set_height(*height);
                                }

                                if let Ok(constraint) = ecm
                                    .component_store_mut()
                                    .borrow_mut_component::<Constraint>(root)
                                {
                                    constraint.set_width(*width);
                                    constraint.set_height(*height);
                                }

                                self.update.set(true);
                            }
                        }
                    }

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
                            self.process_bottom_up_event(&event, ecm);
                        }
                        _ => {}
                    }
                }
            }

            // handle states

            let root = ecm.entity_store().root;

            let theme = ecm
                .component_store()
                .borrow_component::<Theme>(root)
                .unwrap()
                .0
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

                    {
                        let mut widget = context.widget();

                        // handle enabled
                        if let Some(enabled) = widget.try_clone::<Enabled>() {
                            let mut remove_disabled = false;
                            let mut add_disabled = false;
                            if let Some(selector) = widget.try_get::<Selector>() {
                                if enabled.0 && selector.0.pseudo_classes.contains("disabled") {
                                    remove_disabled = true;
                                } else if !enabled.0
                                    && !selector.0.pseudo_classes.contains("disabled")
                                {
                                    add_disabled = true;
                                }
                            }

                            if remove_disabled {
                                remove_selector_from_widget("disabled", &mut context.widget());
                                context.update_theme_properties(context.entity);
                            }

                            if add_disabled {
                                add_selector_to_widget("disabled", &mut context.widget());
                                context.update_theme_properties(context.entity);
                            }
                        }

                        if !self.states.borrow().contains_key(&current_node) {
                            skip = true;
                        }
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

            if shell.adapter().event_queue.len() == 0 {
                break;
            }
        }

        // // update theme properties
        // let theme = ecm
        //     .component_store()
        //     .borrow_component::<Theme>(root)
        //     .unwrap()
        //     .0
        //     .clone();
        // let mut current_node = ecm.entity_store().root;

        // loop {
        //     {
        //         let mut context = Context::new(
        //             current_node,
        //             ecm,
        //             &mut shell,
        //             &theme,
        //             self.render_objects.clone(),
        //             self.layouts.clone(),
        //             self.handlers.clone(),
        //             self.states.clone(),
        //         );

        //         context.update_theme_properties();
        //     }
        //     let mut it = ecm.entity_store().start_node(current_node).into_iter();
        //     it.next();

        //     if let Some(node) = it.next() {
        //         current_node = node;
        //     } else {
        //         break;
        //     }
        // }
    }
}
