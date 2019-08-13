use std::{
    any::TypeId,
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{prelude::*, shell::WindowShell, tree::Tree, utils::*};

pub struct EventSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
}

/// The `EventSystem` pops events from the event queue and delegates the events to the corresponding event handlers of the widgets.
impl EventSystem {
    fn process_top_down_event(
        &self,
        _event: &EventBox,
        _ecm: &mut EntityComponentManager<Tree>,
        _new_events: &mut Vec<EventBox>,
    ) {
    }

    fn process_bottom_up_event(
        &self,
        event: &EventBox,
        ecm: &mut EntityComponentManager<Tree>,
        new_events: &mut Vec<EventBox>,
    ) {
        let mut matching_nodes = vec![];

        let mut current_node = event.source;
        let mut cont = false;

        loop {
            let widget = WidgetContainer::new(current_node, ecm);

            // MouseDownEvent handling
            if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
                if check_mouse_condition(Point::new(event.x, event.y), &widget) {
                    matching_nodes.push(current_node);
                }

                cont = true;
            }

            if !cont {
                // MouseUpEvent handling
                if event.event_type() == TypeId::of::<MouseUpEvent>() {
                    if let Some(pressed) = widget.try_get::<Pressed>() {
                        if pressed.0 {
                            matching_nodes.push(current_node);
                            break;
                        }
                    }
                }

                // Click handling
                if let Ok(event) = event.downcast_ref::<ClickEvent>() {
                    if check_mouse_condition(event.position, &widget) {
                        matching_nodes.push(current_node);
                    }

                    cont = true;
                }

                if !cont {
                    matching_nodes.push(current_node);
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
        let mut new_focused_widget = None;
        let mut disabled_parent = None;
        // let mut new_mouse_over_entity = None;

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

            let mut widget = WidgetContainer::new(*node, ecm);

            if let Some(handlers) = self.handlers.borrow().get(node) {
                for handler in handlers {
                    handled = handler.handle_event(event);

                    if handled {
                        break;
                    }
                }
            }

            // MouseDownEvent handling
            if event.event_type() == TypeId::of::<MouseDownEvent>() {
                if let Some(focused) = widget.try_get_mut::<Focused>() {
                    focused.0 = true;
                    new_focused_widget = Some(*node);
                    self.update.set(true);
                }

                if let Some(pressed) = widget.try_get_mut::<Pressed>() {
                    pressed.0 = true;
                    self.update.set(true);
                }
            }

            // MouseUpEvent handling
            if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
                let mut pressed = false;
                let mut in_mouse_pos = false;

                if check_mouse_condition(Point::new(event.x, event.y), &widget) {
                    in_mouse_pos = true;
                }

                // if in_mouse_pos {
                //     if let Some(selected) = widget.try_get_mut::<Selected>() {
                //         selected.0 = !selected.0;
                //         self.update.set(true);
                //     }
                // }

                if let Some(pres) = widget.try_get_mut::<Pressed>() {
                    pressed = pres.0;
                    pres.0 = false;
                    self.update.set(true);
                }

                if pressed {
                    if in_mouse_pos {
                        new_events.push(EventBox::new(
                            ClickEvent {
                                position: Point::new(event.x, event.y),
                            },
                            EventStrategy::BottomUp,
                            *node,
                        ))
                    }

                    break;
                }
            }

            if handled {
                self.update.set(true);
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

        // remove focus from previous focused entity
        let mut old_focused_widget = None;

        if let Ok(global) = ecm
            .component_store_mut()
            .borrow_mut_component::<Global>(root)
        {
            if let Some(new_focused_widget) = new_focused_widget {
                if let Some(focused_widget) = global.focused_widget {
                    if focused_widget != new_focused_widget {
                        old_focused_widget = Some(focused_widget);
                    }
                }
                global.focused_widget = Some(new_focused_widget);
            }
        }

        if let Some(old_focused_widget) = old_focused_widget {
            if let Ok(focused) = ecm
                .component_store_mut()
                .borrow_mut_component::<Focused>(old_focused_widget)
            {
                focused.0 = false;
                self.update.set(true);
            }
        }
    }
}

impl System<Tree> for EventSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>) {
        let mut shell = self.shell.borrow_mut();
        let adapter = shell.adapter();

        let mut new_events = vec![];
        let root = ecm.entity_store().root;

        loop {
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
                        self.process_top_down_event(&event, ecm, &mut new_events);
                    }
                    EventStrategy::BottomUp => {
                        self.process_bottom_up_event(&event, ecm, &mut new_events);
                    }
                    _ => {}
                }
            }

            adapter.event_queue.append(&mut new_events);

            if adapter.event_queue.len() == 0 {
                break;
            }
        }
    }
}
