use std::any::TypeId;

use crate::{
    prelude::*,
    utils::{Thickness, String16, Brush},
};

use dces::prelude::{Component, Entity, EntityComponentManager};

/// The `WidgetContainer` wraps the entity of a widget and provides access to its properties, its children properties and its parent properties.
pub struct WidgetContainer<'a> {
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    current_node: Entity,
    theme: &'a ThemeValue,
}

impl<'a> WidgetContainer<'a> {
    /// Creates a new widget container for the given `entity`.
    pub fn new(
        root: Entity,
        ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
        theme: &'a ThemeValue,
    ) -> Self {
        WidgetContainer {
            ecm,
            current_node: root,
            theme,
        }
    }

    /// Gets the property.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn get<P>(&self, key: &str) -> &P
    where
        P: Clone + Component,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(key, self.current_node)
        {
            return property;
        }

        panic!(
            "Entity {} does not contain property type {:?} with key: {}",
            self.current_node.0,
            TypeId::of::<P>(),
            key
        );
    }

    /// Gets a mutable reference of the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn get_mut<P>(&mut self, key: &str) -> &mut P
    where
        P: Clone + Component,
    {
        if let Ok(property) = self
            .ecm
            .component_store_mut()
            .borrow_mut_component::<P>(key, self.current_node)
        {
            return property;
        }

        panic!(
            "Entity {} does not contain property type {:?}, with key: {}",
            self.current_node.0,
            TypeId::of::<P>(),
            key
        );
    }

    /// Clones the property. If the property does not exists for the widget the
    /// default of the property value will be returned,
    pub fn clone_or_default<P>(&self, key: &str) -> P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(key, self.current_node)
        {
            return property.clone();
        }

        P::default()
    }

    /// Clones the property.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn clone<P>(&self, key: &str) -> P
    where
        P: Clone + Component,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(key, self.current_node)
        {
            return property.clone();
        }

        panic!(
            "Entity {} does not contain property type {:?}, with key: {}",
            self.current_node.0,
            TypeId::of::<P>(),
            key
        );
    }

    /// Clones the property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_clone<P>(&self, key: &str) -> Option<P>
    where
        P: Clone + Component,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(key, self.current_node)
        {
            return Some(property.clone());
        }

        None
    }

    /// Sets the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn set<P>(&mut self, key: &str, value: P)
    where
        P: Component + Clone,
    {
        if let Ok(property) = self
            .ecm
            .component_store_mut()
            .borrow_mut_component::<P>(key, self.current_node)
        {
            *property = value;
            return;
        }

        panic!(
            "Entity {} does not contain property type {:?}",
            self.current_node.0,
            TypeId::of::<P>()
        );
    }

    /// Returns `true` if the widget has a property of type `P` otherwise `false`.
    pub fn has<P>(&self, key: &str) -> bool
    where
        P: Clone + Component,
    {
        self.ecm
            .component_store()
            .borrow_component::<P>(key, self.current_node)
            .is_ok()
    }

    /// Returns a reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_get<P: Component>(&self, key: &str) -> Option<&P> {
        self.ecm
            .component_store()
            .borrow_component::<P>(key, self.current_node)
            .ok()
    }

    /// Returns a mutable reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_get_mut<P: Component>(&mut self, key: &str) -> Option<&mut P> {
        self.ecm
            .component_store_mut()
            .borrow_mut_component::<P>(key, self.current_node)
            .ok()
    }

    /// Updates the theme by the inner state e.g. `selected` or `pressed`.
    pub fn update_theme_by_state(&mut self, force: bool) {
        if let Some(selector) = self.try_clone::<Selector>("selector") {
            let mut update = false;

            if let Some(focus) = self.try_clone::<bool>("focused") {
                if focus && !selector.0.pseudo_classes.contains("focus") {
                    add_selector_to_widget("focus", self);
                    update = true;
                } else if !focus && selector.0.pseudo_classes.contains("focus") {
                    remove_selector_from_widget("focus", self);
                    update = true;
                }
            }

            if let Some(selected) = self.try_clone::<bool>("selected") {
                if selected && !selector.0.pseudo_classes.contains("selected") {
                    add_selector_to_widget("selected", self);
                    update = true;
                } else if !selected && selector.0.pseudo_classes.contains("selected") {
                    remove_selector_from_widget("selected", self);
                    update = true;
                }
            }

            if let Some(pressed) = self.try_clone::<bool>("pressed") {
                if pressed && !selector.0.pseudo_classes.contains("active") {
                    add_selector_to_widget("active", self);
                    update = true;
                } else if !pressed && selector.0.pseudo_classes.contains("active") {
                    remove_selector_from_widget("active", self);
                    update = true;
                }
            }

            if let Some(enabled) = self.try_clone::<bool>("enabled") {
                if !enabled && !selector.0.pseudo_classes.contains("disabled") {
                    add_selector_to_widget("disabled", self);
                    update = true;
                } else if enabled && selector.0.pseudo_classes.contains("disabled") {
                    remove_selector_from_widget("disabled", self);
                    update = true;
                }
            }

            if let Some(text) = self.try_clone::<String16>("text") {
                if text.len() == 0 && !selector.0.pseudo_classes.contains("empty") {
                    add_selector_to_widget("empty", self);
                    update = true;
                } else if text.len() > 0 && selector.0.pseudo_classes.contains("empty") {
                    remove_selector_from_widget("empty", self);
                    update = true;
                }
            }

            if update || force {
                self.update_properties_by_theme();
            }
        }
    }

    /// Update all properties for the theme.
    pub fn update_properties_by_theme(&mut self) {
        if !self.has::<Selector>("selector") {
            return;
        }

        let selector = self.clone::<Selector>("selector");

        if !selector.0.dirty() {
            return;
        }

        if self.has::<Brush>("foreground") {
            if let Some(color) = self.theme.brush("color", &selector.0) {
                self.set::<Brush>("foreground", Brush::from(color));
            }
        }

        if self.has::<Brush>("background") {
            if let Some(background) = self.theme.brush("background", &selector.0) {
                self.set::<Brush>("background", Brush::from(background));
            }
        }

        if self.has::<Brush>("border_brush") {
            if let Some(border_brush) = self.theme.brush("border-color", &selector.0) {
                self.set::<Brush>("border_brush", Brush::from(border_brush));
            }
        }

        if self.has::<f64>("border_radius") {
            if let Some(radius) = self.theme.float("border-radius", &selector.0) {
                self.set::<f64>("border_radius", f64::from(radius as f64));
            }
        }

        if self.has::<Thickness>("border_width") {
            if let Some(border_width) = self.theme.uint("border-width", &selector.0) {
                self.set::<Thickness>(
                    "border_width",
                    Thickness::from(border_width as f64),
                );
            }
        }

        if self.has::<f64>("font_size") {
            if let Some(size) = self.theme.uint("font-size", &selector.0) {
                self.set::<f64>("font_size", f64::from(size as f64));
            }
        }

        if self.has::<String>("font_family") {
            if let Some(font_family) = self.theme.string("font-family", &selector.0) {
                self.set::<String>("font_family", String::from(font_family));
            }
        }

        if self.has::<Brush>("icon_brush") {
            if let Some(color) = self.theme.brush("icon-color", &selector.0) {
                self.set::<Brush>("icon_brush", Brush::from(color));
            }
        }

        if self.has::<f64>("icon_size") {
            if let Some(size) = self.theme.uint("icon-size", &selector.0) {
                self.set::<f64>("icon_size", f64::from(size as f64));
            }
        }

        if self.has::<String>("icon_family") {
            if let Some(font_family) = self.theme.string("icon-family", &selector.0) {
                self.set::<String>("icon_family", String::from(font_family));
            }
        }

        if let Some(padding) = self.try_clone::<Thickness>("padding") {
            if let Some(pad) = self.theme.uint("padding", &selector.0) {
                let mut padding = padding;
                padding.set_thickness(pad as f64);
                self.set::<Thickness>("padding", padding);
            }
        }

        if let Some(padding) = self.try_clone::<Thickness>("padding") {
            if let Some(left) = self.theme.uint("padding-left", &selector.0) {
                let mut padding = padding;
                padding.set_left(left as f64);
                self.set::<Thickness>("padding", padding);
            }
        }

        if let Some(padding) = self.try_clone::<Thickness>("padding") {
            if let Some(top) = self.theme.uint("padding-top", &selector.0) {
                let mut padding = padding;
                padding.set_top(top as f64);
                self.set::<Thickness>("padding", padding);
            }
        }

        if let Some(padding) = self.try_clone::<Thickness>("padding") {
            if let Some(right) = self.theme.uint("padding-right", &selector.0) {
                let mut padding = padding;
                padding.set_right(right as f64);
                self.set::<Thickness>("padding", padding);
            }
        }

        if let Some(padding) = self.try_clone::<Thickness>("padding") {
            if let Some(bottom) = self.theme.uint("padding-bottom", &selector.0) {
                let mut padding = padding;
                padding.set_bottom(bottom as f64);
                self.set::<Thickness>("padding", padding);
            }
        }

        // todo padding, icon_margin

        self.get_mut::<Selector>("selector").0.set_dirty(true);
    }
}
