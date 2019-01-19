use std::{any::TypeId, collections::HashMap, rc::Rc};

use dces::prelude::{Component, ComponentBox};

use crate::{
    enums::{ParentType, Visibility},
    event::EventHandler,
    layout::{Layout, RootLayout},
    properties::{Bounds, Point},
    theme::UpdateableShape,
};

use super::{SharedProperty, State};

/// `Template` is used to define the inner structure of a widget.
/// Intern it is used to create an entity with components for the widget.
pub struct Template {
    pub children: Vec<Template>,
    pub parent_type: ParentType,
    pub state: Option<Rc<dyn State>>,
    pub event_handlers: Vec<Rc<dyn EventHandler>>,
    pub shape: Option<Box<dyn UpdateableShape>>,
    pub layout: Box<dyn Layout>,
    pub properties: HashMap<TypeId, ComponentBox>,
    pub shared_properties: HashMap<TypeId, SharedProperty>,
    pub debug_name: String,
}

impl Default for Template {
    fn default() -> Self {
        let mut properties = HashMap::new();

        // register default set of widget properties
        properties.insert(TypeId::of::<Bounds>(), ComponentBox::new::<Bounds>(Bounds::default()));
        properties.insert(TypeId::of::<Point>(), ComponentBox::new::<Point>(Point::default()));
        properties.insert(TypeId::of::<Visibility>(), ComponentBox::new::<Visibility>(Visibility::default()));

        Template {
            children: vec![],
            parent_type: ParentType::None,
            state: None,
            shape: None,
            event_handlers: vec![],
            layout: Box::new(RootLayout),
            properties,
            shared_properties: HashMap::new(),
            debug_name: String::default(),
        }
    }
}

impl Template {
    /// Set the debug name of the widget. It is used to print the name of the widget while widget creation if `debug_flag` on window is set to `true`.
    pub fn with_debug_name(mut self, name: impl Into<String>) -> Self {
        self.debug_name = name.into();
        self
    }

    /// Define the `parent_type` of a widget. The `parent_type` should be set before start adding children.
    pub fn as_parent_type(mut self, parent_type: ParentType) -> Self {
        self.parent_type = parent_type;
        self
    }

    /// Add a child to the widget template. If `parent_type` is set to `ParentType::None` this method do nothing. If `parent_type` is set to `ParentType::Single` only on child could
    /// be added. Every call of this method will overwrite the existing `child`. If `parent_type`is set to `ParentType::Multiple` any number of children could be added to the template.
    pub fn with_child(mut self, child: Template) -> Self {
        match self.parent_type {
            ParentType::Single => {
                self.children.clear();
                self.children.push(child);
            }
            ParentType::Multi => {
                self.children.push(child);
            }
            _ => return self,
        }

        self
    }

    /// Used to add a `state' to the template. Only one `state` can be added.
    pub fn with_state(mut self, state: Rc<dyn State>) -> Self {
        self.state = Some(state);
        self
    }

    /// Add an event handler to the template. Multiple event handlers can be added.
    pub fn with_event_handler(mut self, handler: impl Into<Rc<dyn EventHandler>>) -> Self {
        self.event_handlers.push(handler.into());
        self
    }

    /// Used to add a `shape' to the template. Only one `shape` can be added.
    pub fn with_shape(mut self, shape: impl Into<Box<dyn UpdateableShape>>) -> Self {
        self.shape = Some(shape.into());
        self
    }

    /// Used to add a `layout' to the template. Only one `layout` can be added.
    pub fn with_layout(mut self, layout: impl Into<Box<dyn Layout>>) -> Self {
        self.layout = layout.into();
        self
    }

    /// Used to register a `property' for the template. Only one property per type can be registered.
    /// If a shared property of the same type exists the value of the shared property will be set to
    /// the given property's value.
    pub fn with_property<C: Component>(mut self, property: C) -> Self {
        let type_id = TypeId::of::<C>();

        if !self.shared_properties.contains_key(&type_id) {
            self.properties
                .insert(TypeId::of::<C>(), ComponentBox::new::<C>(property));
        } else {
            self.shared_properties
                .get_mut(&type_id)
                .unwrap()
                .update_property(property);
        }

        self
    }

    /// Used to register a shared property for the template. Only one shared property per type can be registered.
    /// If a property of the same type exists, it will be replaced by the given shared property.
    pub fn with_shared_property(mut self, property: SharedProperty) -> Self {
        if self.properties.contains_key(&property.type_id) {
            self.properties.remove(&property.type_id);
        }

        if self.shared_properties.contains_key(&property.type_id) {
            let type_id = property.type_id;

            self.shared_properties.get_mut(&type_id).unwrap().property = None;
            self.shared_properties
                .get_mut(&type_id)
                .unwrap()
                .source_chain
                .borrow_mut()
                .push(
                    property.source_chain.borrow()[property.source_chain.borrow().len() - 1]
                        .clone(),
                );
        } else {
            self.shared_properties.insert(property.type_id, property);
        }

        self
    }
}
