use std::any::TypeId;

use crate::{prelude::*, utils::*};

use dces::prelude::{Component, Entity, EntityComponentManager};

/// The `WidgetContainer` wraps the entity of a widget and provides access to its properties, its children properties and its parent properties.
pub struct WidgetContainer<'a> {
    ecm: &'a mut EntityComponentManager<Tree>,
    current_node: Entity,
    theme: &'a ThemeValue,
}

impl<'a> WidgetContainer<'a> {
    /// Creates a new widget container for the given `entity`.
    pub fn new(
        root: Entity,
        ecm: &'a mut EntityComponentManager<Tree>,
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
    pub fn get<P>(&self) -> &P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
        {
            return property;
        }

        panic!(
            "Entity {} does not contain property type {:?}",
            self.current_node.0,
            TypeId::of::<P>()
        );
    }

    /// Gets a mutable reference of the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn get_mut<P>(&mut self) -> &mut P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store_mut()
            .borrow_mut_component::<P>(self.current_node)
        {
            return property;
        }

        panic!(
            "Entity {} does not contain property type {:?}",
            self.current_node.0,
            TypeId::of::<P>()
        );
    }

    /// Clones the property. If the property does not exists for the widget the
    /// default of the property value will be returned,
    pub fn clone_or_default<P>(&self) -> P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
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
    pub fn clone<P>(&self) -> P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
        {
            return property.clone();
        }

        panic!(
            "Entity {} does not contain property type {:?}",
            self.current_node.0,
            TypeId::of::<P>()
        );
    }

    /// Clones the property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_clone<P>(&self) -> Option<P>
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
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
    pub fn set<P>(&mut self, value: P)
    where
        P: Component + Default + Clone,
    {
        if let Ok(property) = self
            .ecm
            .component_store_mut()
            .borrow_mut_component::<P>(self.current_node)
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
    pub fn has<P>(&self) -> bool
    where
        P: Clone + Component + Default,
    {
        self.ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
            .is_ok()
    }

    /// Returns a reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_get<P: Component + Default>(&self) -> Option<&P> {
        self.ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
            .ok()
    }

    /// Returns a mutable reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_get_mut<P: Component + Default>(&mut self) -> Option<&mut P> {
        self.ecm
            .component_store_mut()
            .borrow_mut_component::<P>(self.current_node)
            .ok()
    }

    /// Updates the theme by the inner state e.g. `selected` or `pressed`.
    pub fn update_theme_by_state(&mut self) {
        let selector = self.clone::<Selector>();

        if let Some(focus) = self.try_clone::<Focused>() {
            if focus.0 && !selector.0.pseudo_classes.contains("focus") {
                add_selector_to_widget("focus", self);
            } else if !focus.0 && selector.0.pseudo_classes.contains("focus") {
                remove_selector_from_widget("focus", self)
            }
        }

        if let Some(selected) = self.try_clone::<Selected>() {
            if selected.0 && !selector.0.pseudo_classes.contains("selected") {
                add_selector_to_widget("selected", self);
            } else if !selected.0 && selector.0.pseudo_classes.contains("selected") {
                remove_selector_from_widget("selected", self)
            }
        }

        if let Some(pressed) = self.try_clone::<Pressed>() {
            if pressed.0 && !selector.0.pseudo_classes.contains("active") {
                add_selector_to_widget("active", self);
            } else if !pressed.0 && selector.0.pseudo_classes.contains("active") {
                remove_selector_from_widget("active", self)
            }
        }

        self.update_properties_by_theme();
    }

    /// Update all properties for the theme.
    pub fn update_properties_by_theme(&mut self) {
        if !self.has::<Selector>() {
            return;
        }

        let selector = self.clone::<Selector>();

        if !selector.0.dirty() {
            return;
        }

        if self.has::<Foreground>() {
            if let Some(color) = self.theme.brush("color", &selector.0) {
                self.set::<Foreground>(Foreground::from(color));
            }
        }

        if self.has::<Background>() {
            if let Some(background) = self.theme.brush("background", &selector.0) {
                self.set::<Background>(Background::from(background));
            }
        }

        if self.has::<BorderBrush>() {
            if let Some(border_color) = self.theme.brush("border-color", &selector.0) {
                self.set::<BorderBrush>(BorderBrush::from(border_color));
            }
        }

        if self.has::<BorderRadius>() {
            if let Some(radius) = self.theme.float("border-radius", &selector.0) {
                self.set::<BorderRadius>(BorderRadius::from(radius as f64));
            }
        }

        if self.has::<BorderThickness>() {
            if let Some(border_width) = self.theme.uint("border-width", &selector.0) {
                self.set::<BorderThickness>(BorderThickness::from(border_width as f64));
            }
        }

        if self.has::<FontSize>() {
            if let Some(size) = self.theme.uint("font-size", &selector.0) {
                self.set::<FontSize>(FontSize::from(size as f64));
            }
        }

        if self.has::<Font>() {
            if let Some(font_family) = self.theme.string("font-family", &selector.0) {
                self.set::<Font>(Font::from(font_family));
            }
        }

        if self.has::<IconBrush>() {
            if let Some(color) = self.theme.brush("icon-color", &selector.0) {
                self.set::<IconBrush>(IconBrush::from(color));
            }
        }

        if self.has::<IconSize>() {
            if let Some(size) = self.theme.uint("icon-size", &selector.0) {
                self.set::<IconSize>(IconSize::from(size as f64));
            }
        }

        if self.has::<IconFont>() {
            if let Some(font_family) = self.theme.string("icon-family", &selector.0) {
                self.set::<IconFont>(IconFont::from(font_family));
            }
        }

        if let Some(padding) = self.try_clone::<Padding>() {
            if let Some(pad) = self.theme.uint("padding", &selector.0) {
                let mut padding = padding;
                padding.set_thickness(pad as f64);
                self.set::<Padding>(padding);
            }
        }

        if let Some(padding) = self.try_clone::<Padding>() {
            if let Some(left) = self.theme.uint("padding-left", &selector.0) {
                let mut padding = padding;
                padding.set_left(left as f64);
                self.set::<Padding>(padding);
            }
        }

        if let Some(padding) = self.try_clone::<Padding>() {
            if let Some(top) = self.theme.uint("padding-top", &selector.0) {
                let mut padding = padding;
                padding.set_top(top as f64);
                self.set::<Padding>(padding);
            }
        }

        if let Some(padding) = self.try_clone::<Padding>() {
            if let Some(right) = self.theme.uint("padding-right", &selector.0) {
                let mut padding = padding;
                padding.set_right(right as f64);
                self.set::<Padding>(padding);
            }
        }

        if let Some(padding) = self.try_clone::<Padding>() {
            if let Some(bottom) = self.theme.uint("padding-bottom", &selector.0) {
                let mut padding = padding;
                padding.set_bottom(bottom as f64);
                self.set::<Padding>(padding);
            }
        }

        // todo padding, icon_margin

        self.get_mut::<Selector>().0.set_dirty(true);
    }
}
