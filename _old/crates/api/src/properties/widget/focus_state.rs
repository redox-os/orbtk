use crate::{theming::Selector, widget_base::Context};

use dces::prelude::Entity;

/// Contains the state information of the current focused element.
///
/// Provides methods to request and remove focus.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct FocusState {
    focused_entity: Option<Entity>,
}

impl FocusState {
    /// Request focus for the given entity.
    pub fn request_focus(&mut self, entity: impl Into<Entity>, ctx: &mut Context) {
        let entity = entity.into();

        if (self.focused_entity.is_some() && self.focused_entity.unwrap() == entity)
            || !*ctx.get_widget(entity).get::<bool>("enabled")
        {
            return;
        }

        if let Some(old_focused_element) = self.focused_entity {
            let mut old_focused_element = ctx.get_widget(old_focused_element);

            old_focused_element.set("focused", false);
            old_focused_element
                .get_mut::<Selector>("selector")
                .remove_all_similar_states("focused");
            old_focused_element.update(false);
        }

        self.focused_entity = Some(entity);

        if ctx.get_widget(entity).has::<bool>("focused") {
            let mut focused_element = ctx.get_widget(entity);

            focused_element.set("focused", true);
            focused_element
                .get_mut::<Selector>("selector")
                .push_state("focused");
            focused_element.update(false);
        }
    }

    /// Remove the focus of the given entity. If the given entity is not the focused entity nothing will happen.
    pub fn remove_focus(&mut self, entity: impl Into<Entity>, ctx: &mut Context) {
        let entity = entity.into();

        if let Some(old_focused_element) = self.focused_entity {
            if old_focused_element != entity {
                return;
            }
            let mut old_focused_element = ctx.get_widget(old_focused_element);
            old_focused_element.set("focused", false);
            old_focused_element
                .get_mut::<Selector>("selector")
                .remove_all_similar_states("focused");
            old_focused_element.update(false);
        }

        self.focused_entity = None;
    }

    /// Returns `true` if the given entity is focused.
    pub fn has_focus(&self, entity: impl Into<Entity>) -> bool {
        self.focused_entity.is_some() && self.focused_entity.unwrap() == entity.into()
    }

    /// Returns a reference to the current focused entity.
    pub fn focused_entity(&self) -> &Option<Entity> {
        &self.focused_entity
    }
}
