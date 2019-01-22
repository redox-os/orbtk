use std::{
    cell::RefCell,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{
    application::{Global, Tree},
    backend::Backend,
    properties::{Constraint, Margin, Padding},
    theme::{Selector, Theme},
};

/// This system is used to initializes the widgets.
pub struct InitSystem {
    pub backend: Rc<RefCell<dyn Backend>>,
}

impl InitSystem {
    // init css ids.
    fn init_id(&self, node: Entity, ecm: &mut EntityComponentManager) {
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

    // Read all initial data from css
    fn read_init_from_theme(&self, node: Entity, ecm: &mut EntityComponentManager, theme: &Theme) {
        let mut margin = Margin::default();
        let mut padding = Padding::default();
        let mut constraint = Constraint::default();

        // todo: update widget by selector method!!!

        if let Ok(selector) = ecm.borrow_component::<Selector>(node) {
            let pad = theme.uint("padding", selector) as i32;

            if pad > 0 {
                padding.left = pad;
                padding.top = pad;
                padding.right = pad;
                padding.bottom = pad;
            } else {
                padding.left = theme.uint("padding-left", selector) as i32;
                padding.top = theme.uint("padding-top", selector) as i32;
                padding.right = theme.uint("padding-right", selector) as i32;
                padding.bottom = theme.uint("padding-bottom", selector) as i32;
            }

            let mar = theme.uint("margin", selector) as i32;

            if mar > 0 {
                margin.left = mar;
                margin.top = mar;
                margin.right = mar;
                margin.bottom = mar;
            } else {
                margin.left = theme.uint("margin-left", selector) as i32;
                margin.top = theme.uint("margin-top", selector) as i32;
                margin.right = theme.uint("margin-right", selector) as i32;
                margin.bottom = theme.uint("margin-bottom", selector) as i32;
            }

            constraint.min_width = theme.uint("min-width", selector) as u32;
            constraint.max_width = theme.uint("max_width-width", selector) as u32;
            constraint.min_height = theme.uint("min_height-width", selector) as u32;
            constraint.max_height = theme.uint("min-max_height", selector) as u32;
            constraint.width = theme.uint("width", selector) as u32;
            constraint.height = theme.uint("height", selector) as u32;
        }

        ecm.register_component(node, padding);
        ecm.register_component(node, margin);
        ecm.register_component(node, constraint);
    }
}

impl System<Tree> for InitSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        let mut backend = self.backend.borrow_mut();
        let context = backend.state_context();

        // init css ids
        for node in tree.into_iter() {
            self.init_id(node, ecm);
            self.read_init_from_theme(node, ecm, context.theme);
        }
    }
}
