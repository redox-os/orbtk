use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::{Entity, EntityComponentManager, System};

use crate::{
    application::{Global, Tree},
    core::Backend,
    properties::{Enabled, Focused, Pressed, Selected},
    theme::Selector,
    widget::{
        add_selector_to_widget, remove_selector_from_widget, Context, State, WidgetContainer,
    },
};

/// The `StateSystem` calls the update methods of widget states.
pub struct StateSystem {
    pub backend: Rc<RefCell<dyn Backend>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub update: Rc<Cell<bool>>,
    pub is_init: Cell<bool>,
}

impl StateSystem {
    fn init(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        for node in tree.into_iter() {
            // Add css id to global id map.
            let id = if let Ok(selector) = ecm.borrow_component::<Selector>(node) {
                if let Some(id) = &selector.id {
                    Some((node, id.clone()))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some((entity, id)) = id {
                if let Ok(global) = ecm.borrow_mut_component::<Global>(0) {
                    global.id_map.insert(id, entity);
                }
            }
        }

        self.is_init.set(true);
    }

    fn has_default_flags(&self, widget: &WidgetContainer<'_>) -> bool {
        if let Ok(_) = widget.borrow_property::<Enabled>() {
            return true;
        }

        if let Ok(_) = widget.borrow_property::<Pressed>() {
            return true;
        }

        if let Ok(_) = widget.borrow_property::<Focused>() {
            return true;
        }

        if let Ok(_) = widget.borrow_property::<Selected>() {
            return true;
        }

        return false;
    }

    // Used to updates default states like Pressed, Focused and Enabled.
    fn update_default_states(&self, widget: &mut WidgetContainer<'_>) {
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

impl System<Tree> for StateSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.is_init.get() {
            self.init(tree, ecm);
        }

        if !self.update.get() {
            return;
        }

        let mut backend = self.backend.borrow_mut();
        let state_context = backend.state_context();
        let mut context = Context::new(tree.root, ecm, tree, &state_context.theme);

        for node in tree.into_iter() {
            context.entity = node;
            {
                let mut widget = context.widget();

                let has_default_flags = self.has_default_flags(&widget);
                if !has_default_flags && !self.states.borrow().contains_key(&node) {
                    continue;
                }

                if has_default_flags {
                    self.update_default_states(&mut widget);
                }
            }

            if let Some(state) = self.states.borrow().get(&node) {
                state.update(&mut context);
            }
        }
    }
}

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
pub struct PostLayoutStateSystem {
    pub backend: Rc<RefCell<dyn Backend>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub update: Rc<Cell<bool>>,
}

impl System<Tree> for PostLayoutStateSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.update.get() {
            return;
        }

        let mut backend = self.backend.borrow_mut();
        let state_context = backend.state_context();
        let mut context = Context::new(tree.root, ecm, tree, &state_context.theme);

        for (node, state) in &*self.states.borrow() {
            context.entity = *node;

            state.update_post_layout(&mut context);
        }
    }
}
