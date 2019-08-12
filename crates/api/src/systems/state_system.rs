use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{prelude::*, shell::WindowShell, tree::Tree};

/// The `StateSystem` calls the update methods of widget states.
pub struct StateSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
}

impl StateSystem {
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

impl System<Tree> for StateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>) {
        if !self.update.get() || !self.running.get() {
            return;
        }

        let root = ecm.entity_store().root;

        let theme = ecm
            .component_store()
            .borrow_component::<Theme>(root)
            .unwrap()
            .0
            .clone();
        let window_shell = &mut self.shell.borrow_mut();
        let mut current_node = root;

        loop {
            let mut skip = false;

            {
                let mut context = Context::new(
                    current_node,
                    ecm,
                    window_shell,
                    &theme,
                    self.render_objects.clone(),
                    self.layouts.clone(),
                    self.handlers.clone(),
                    self.states.clone(),
                );

                {
                    let mut widget = context.widget();

                    let has_default_flags = self.has_default_flags(&widget);
                    if !has_default_flags && !self.states.borrow().contains_key(&current_node) {
                        skip = true;
                    }

                    if has_default_flags {
                        self.update_default_states(&mut widget);
                    }
                }

                if !skip {
                    if let Some(state) = self.states.borrow().get(&current_node) {
                        state.update(&mut context);
                    }
                }

                context.update_theme_properties();
            }

            let mut it = ecm.entity_store().start_node(current_node).into_iter();
            it.next();

            if let Some(node) = it.next() {
                current_node = node;
            } else {
                break;
            }
        }
    }
}

/// The `PostLayoutStateSystem` calls the update_post_layout methods of widget states.
pub struct PostLayoutStateSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
}

impl System<Tree> for PostLayoutStateSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>) {
        if !self.update.get() || !self.running.get() {
            return;
        }

        let root = ecm.entity_store().root;

        let window_shell = &mut self.shell.borrow_mut();
        let theme = ecm
            .component_store()
            .borrow_component::<Theme>(root)
            .unwrap()
            .0
            .clone();

        let mut context = Context::new(
            root,
            ecm,
            window_shell,
            &theme,
            self.render_objects.clone(),
            self.layouts.clone(),
            self.handlers.clone(),
            self.states.clone(),
        );

        for (node, state) in &*self.states.borrow() {
            context.entity = *node;

            state.update_post_layout(&mut context);

            // Handle messages.
            {
                // todo fix messages.
                // for (entity, messages) in context.messages().iter() {
                //     if let Some(state) = self.states.borrow().get(&entity) {
                //         context.entity = *entity;
                //         state.receive_messages(&mut context, &messages);
                //     }
                // }
            }
        }
    }
}
