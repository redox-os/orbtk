use dces::{Entity, EntityComponentManager};

use super::{KeyChain, WidgetContainer};
use application::Tree;
use theme::Theme;

/// The `Context` is provides acces for the states to objects they could work with.
pub struct Context<'a> {
    ecm: &'a mut EntityComponentManager,
    tree: &'a Tree,
    pub entity: Entity,
    pub theme: &'a Theme,
}

impl<'a> Context<'a> {
    /// Creates a new container.
    pub fn new(
        entity: Entity,
        ecm: &'a mut EntityComponentManager,
        tree: &'a Tree,
        theme: &'a Theme,
    ) -> Self {
        Context {
            entity,
            ecm,
            tree,
            theme,
        }
    }

    /// Returns the widget of the current state context.
    pub fn widget(&mut self) -> WidgetContainer {
        WidgetContainer::new(self.entity, &mut self.ecm)
    }

    /// Returns a child of the widget of the current state referenced by `key`.
    /// If the current widget has no `KeyChain` or there is no child with the given `key` None will returned.
    pub fn widget_from_key(&mut self, key: &str) -> Option<WidgetContainer> {
        let mut entity = None;

        if let Ok(key_chain) = self.ecm.borrow_component::<KeyChain>(self.entity) {
            if let Some(en) = key_chain.get(key) {
                entity = Some(en);
            }
        }

        if let Some(entity) = entity {
            return Some(WidgetContainer::new(entity, &mut self.ecm));
        }

        None
    }

    /// Returns the child of the current widget.
    /// If the index is out of the children index bounds or the widget has no children None will be returned.
    pub fn widget_from_child_index(&mut self, index: usize) -> Option<WidgetContainer> {
        if index >= self.tree.children[&self.entity].len() {
            return None;
        }

        Some(WidgetContainer::new(
            self.tree.children[&self.entity][index],
            &mut self.ecm,
        ))
    }

    /// Returns the parent of the current widget.
    /// If the current widget is the root None will be returned.
    pub fn parent_widget(&mut self) -> Option<WidgetContainer> {
        if self.tree.parent[&self.entity] == 0 {
            return None;
        }

        Some(WidgetContainer::new(
            self.tree.parent[&self.entity],
            &mut self.ecm,
        ))
    }
}
