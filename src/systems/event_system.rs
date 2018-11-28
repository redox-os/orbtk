use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::collections::BTreeMap;

use application::Tree;
use backend::Backend;
use dces::{Entity, EntityComponentManager, System};
use event::{
    check_mouse_condition, ClickEvent, EventBox, EventHandler, EventStrategy, 
    MouseDownEvent, MouseUpEvent,
};
use widget::WidgetContainer;
use structs::{Enabled, Focused, Pressed};
use Global;

pub struct EventSystem {
    pub backend: Rc<RefCell<Backend>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
}

/// The `EventSystem` pops events from the event queue and delegates the events to the corresponding event handlers of the widgets.
impl EventSystem {
    fn process_top_down_event(
        &self,
        _event: &EventBox,
        _tree: &Tree,
        _ecm: &mut EntityComponentManager,
        _new_events: &mut Vec<EventBox>,
    ) {
    }

    fn process_bottom_up_event(
        &self,
        event: &EventBox,
        tree: &Tree,
        ecm: &mut EntityComponentManager,
        new_events: &mut Vec<EventBox>,
    ) {
        let mut matching_nodes = vec![];

        for node in tree.into_iter() {
            // MouseDownEvent handling
            if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
                if check_mouse_condition(event.position, &WidgetContainer::new(node, ecm, tree)) {
                    matching_nodes.push(node);
                }

                continue;
            }

            // MouseUpEvent handling
            if event.event_type() == TypeId::of::<MouseUpEvent>() {
                if let Ok(pressed) =
                    WidgetContainer::new(node, ecm, tree).borrow_property::<Pressed>()
                {
                    if pressed.0 {
                        matching_nodes.push(node);
                        break;
                    }
                }
            }

            // Click handling
            if let Ok(event) = event.downcast_ref::<ClickEvent>() {
                if check_mouse_condition(event.position, &WidgetContainer::new(node, ecm, tree)) {
                    matching_nodes.push(node);
                }

                continue;
            }

            matching_nodes.push(node);
        }

        let mut handled = false;
        let mut new_focused_widget = None;
        let mut disabled_parent = None;
        // let mut new_mouse_over_entity = None;

        for node in matching_nodes.iter().rev() {
            if let Some(dp) = disabled_parent {
                if tree.parent[&node] == dp {
                    disabled_parent = Some(*node);
                    continue;
                } else {
                    disabled_parent = None;
                }
            }

            if let Ok(enabled) = ecm.borrow_component::<Enabled>(*node) {
                if !enabled.0 {
                    disabled_parent = Some(*node);
                    continue;
                }
            }
            let mut widget = WidgetContainer::new(*node, ecm, tree);

            if let Some(handlers) = self.handlers.borrow().get(node) {
                for handler in handlers {
                    handled = handler.handle_event(event, &mut widget);

                    if handled {
                        break;
                    }
                }
            }

            // MouseDownEvent handling
            if event.event_type() == TypeId::of::<MouseDownEvent>() {
                if let Ok(focused) = widget.borrow_mut_property::<Focused>() {
                    focused.0 = true;
                    new_focused_widget = Some(*node);
                    self.update.set(true);
                    break;
                }

                if let Ok(pressed) = widget.borrow_mut_property::<Pressed>() {
                    pressed.0 = true;
                    self.update.set(true);
                    break;
                }
            }

            // MouseUpEvent handling
            if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
                let mut pressed = false;

                if let Ok(pres) = widget.borrow_mut_property::<Pressed>() {
                    pressed = pres.0;
                    pres.0 = false;
                    self.update.set(true);
                }

                if pressed {
                    if check_mouse_condition(event.position, &widget) {
                        new_events.push(EventBox::new(
                            ClickEvent {
                                position: event.position,
                            },
                            EventStrategy::BottomUp,
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

        // remove focus from previes focues entity
        let mut old_focused_widget = None;

        if let Ok(global) = ecm.borrow_mut_component::<Global>(tree.root) {
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
            if let Ok(focused) = ecm.borrow_mut_component::<Focused>(old_focused_widget) {
                focused.0 = false;
                self.update.set(true);
            }
        }
    }
}

impl System<Tree> for EventSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut backend = self.backend.borrow_mut();
        let event_context = backend.event_context();

        let mut new_events = vec![];

        for event in event_context.event_queue.borrow_mut().into_iter() {
            match event.strategy {
                EventStrategy::TopDown => {
                    self.process_top_down_event(&event, tree, ecm, &mut new_events);
                }
                EventStrategy::BottomUp => {
                    self.process_bottom_up_event(&event, tree, ecm, &mut new_events);
                }
                _ => {}
            }
        }

        event_context
            .event_queue
            .borrow_mut()
            .append(&mut new_events);
    }
}
