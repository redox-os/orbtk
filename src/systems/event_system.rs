use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::collections::BTreeMap;

use dces::{Entity, EntityComponentManager, System};

use backend::Backend;
use event::{
    check_mouse_condition, EventBox, EventHandler, EventStrategy, Focused, MouseDownEvent,
    MouseUpEvent, Pressed,
};
// use structs::Point;
use tree::Tree;
use widget::WidgetContainer;

pub struct EventSystem {
    pub backend: Rc<RefCell<Backend>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
}

impl EventSystem {
    fn process_top_down_event(
        &self,
        _event: &EventBox,
        _tree: &Tree,
        _ecm: &mut EntityComponentManager,
    ) {
    }

    fn process_bottom_up_event(
        &self,
        event: &EventBox,
        tree: &Tree,
        ecm: &mut EntityComponentManager,
    ) {
        let mut matching_nodes = vec![];

        for node in tree.into_iter() {
            // MouseDownEvent handling
            let mut widget = WidgetContainer::new(node, ecm, tree);
            if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
                if check_mouse_condition(event.position, &widget) {
                    matching_nodes.push(node);
                }

                continue;
            }

            // MouseUpEvent handling
            if event.event_type() == TypeId::of::<MouseUpEvent>() {
                if let Ok(pressed) = widget.borrow_property::<Pressed>() {
                    if pressed.0 {
                        matching_nodes.push(node);
                        break;
                    }
                }
            }

            matching_nodes.push(node);
        }

        let mut handled = false;

        for node in matching_nodes.iter().rev() {
            let mut widget = WidgetContainer::new(*node, ecm, tree);

            if let Some(handlers) = self.handlers.borrow().get(node) {
                for handler in handlers {
                    handled = handler.handle_event(event, &mut widget);

                    if handled {
                        self.update.set(true);
                        break;
                    }
                }
            }

            // MouseDownEvent handling
            if event.event_type() == TypeId::of::<MouseDownEvent>() {
                if let Ok(focused) = widget.borrow_mut_property::<Focused>() {
                    focused.0 = true;
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
            if event.event_type() == TypeId::of::<MouseUpEvent>() {
                if let Ok(pressed) = widget.borrow_mut_property::<Pressed>() {
                    pressed.0 = false;
                    self.update.set(true);
                    break;
                }
            }

            if handled {
                break;
            }
        }
    }
}

impl System<Tree> for EventSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut backend = self.backend.borrow_mut();
        let event_context = backend.event_context();

        for event in event_context.event_queue.borrow_mut().into_iter() {
            match event.strategy {
                EventStrategy::TopDown => {
                    self.process_top_down_event(&event, tree, ecm);
                }
                EventStrategy::BottomUp => {
                    self.process_bottom_up_event(&event, tree, ecm);
                }
                _ => {}
            }
        }
    }
}
