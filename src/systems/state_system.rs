use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::collections::BTreeMap;

use dces::{Entity, EntityComponentManager, System};

use application::Tree;
use properties::{Enabled, Focused, Pressed, Selected};
use widget::{Context, add_selector_to_widget, remove_selector_from_widget, State, WidgetContainer};

/// The `StateSystem` calls the update methods of widget states.
pub struct StateSystem {
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
    pub update: Rc<Cell<bool>>,
}

impl StateSystem {
    fn has_default_flags(&self, node: Entity, ecm: &EntityComponentManager) -> bool {
        if let Ok(_) = ecm.borrow_component::<Enabled>(node) {
            return true;
        }

        if let Ok(_) = ecm.borrow_component::<Pressed>(node) {
            return true;
        }

        if let Ok(_) = ecm.borrow_component::<Focused>(node) {
            return true;
        }

        if let Ok(_) = ecm.borrow_component::<Selected>(node) {
            return true;
        }

        return false;
    }

    // Used to updates default states like Pressed, Focused and Enabled.
    fn update_default_states(&self, widget: &mut WidgetContainer) {
        let mut enabled = (false, false);
        if let Ok(en) = widget.borrow_property::<Enabled>() {
            enabled = (true, en.0);
        }

        if enabled.0 {
            self.update_default_state(!enabled.1, "disabled", widget);
        }

        let mut pressed = (false, false);
        if let Ok(pres) = widget.borrow_mut_property::<Pressed>() {
            pressed = (true, pres.0);
        }

        if pressed.0 {
            self.update_default_state(pressed.1, "active", widget);
        }

        let mut focused = (false, false);
        if let Ok(foc) = widget.borrow_mut_property::<Focused>() {
            focused = (true, foc.0);
        }

        if focused.0 {
            self.update_default_state(focused.1, "focus", widget);
        }

        let mut selected = (false, false);
        if let Ok(sel) = widget.borrow_mut_property::<Selected>() {
            selected = (true, sel.0);
        }

        if selected.0 {
            self.update_default_state(selected.1, "selected", widget);
        }
    }

    // Updates the peseudo class of a widget by the given state.
    fn update_default_state(&self, state: bool, pseudo_class: &str, widget: &mut WidgetContainer) {
        if state {
            add_selector_to_widget(pseudo_class, widget)
        } else {
            remove_selector_from_widget(pseudo_class, widget);
        }
    }
}

impl System<Tree> for StateSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.update.get() {
            return;
        }

        for node in tree.into_iter() {
            let has_default_flags = self.has_default_flags(node, ecm);
            if !has_default_flags && !self.states.borrow().contains_key(&node) {
                continue;
            }

            let mut widget = WidgetContainer::new(node, ecm, tree);

            if has_default_flags {
                self.update_default_states(&mut widget);
            }

            if let Some(state) = self.states.borrow().get(&node) {
                state.update(&mut Context { widget: &mut widget });
            }
        }
    }
}

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
pub struct PostLayoutStateSystem {
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
}

impl System<Tree> for PostLayoutStateSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        for (node, state) in &*self.states.borrow() {
            let mut widget = WidgetContainer::new(*node, ecm, tree);

            state.update_post_layout(&mut Context { widget: &mut widget });
        }
    }
}
