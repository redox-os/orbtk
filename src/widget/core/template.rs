use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;

use dces::{Component, ComponentBox};


use enums::ParentType;
use event::EventHandler;
use layout_object::{LayoutObject, RootLayoutObject};
use render_object::RenderObject;
use super::{State, SharedProperty};


/// `Template` is used to define the inner structure of a widget.
/// Intern it is used to create an entity with components for the widget.
pub struct Template {
    pub children: Vec<Template>,
    pub parent_type: ParentType,
    pub state: Option<Rc<State>>,
    pub event_handlers: Vec<Rc<EventHandler>>,
    pub render_object: Option<Box<RenderObject>>,
    pub layout_object: Box<LayoutObject>,
    pub properties: HashMap<TypeId, ComponentBox>,
    pub shared_properties: HashMap<TypeId, SharedProperty>,
    pub debug_name: String,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            children: vec![],
            parent_type: ParentType::None,
            state: None,
            event_handlers: vec![],
            render_object: None,
            layout_object: Box::new(RootLayoutObject),
            properties: HashMap::new(),
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
    pub fn with_state(mut self, state: Rc<State>) -> Self {
        self.state = Some(state);
        self
    }

    /// Add an event handler to the template. Multiple event handlers can be added.
    pub fn with_event_handler(mut self, handler: impl Into<Rc<dyn EventHandler>>) -> Self {
        self.event_handlers.push(handler.into());
        self
    }

    /// Used to add a `render_object' to the template. Only one `render_object` can be added.
    pub fn with_render_object(mut self, render_object: impl Into<Box<dyn RenderObject>>) -> Self {
        self.render_object = Some(render_object.into());
        self
    }

    /// Used to add a `layout_object' to the template. Only one `layout_object` can be added.
    pub fn with_layout_object(mut self, layout_object: impl Into<Box<dyn LayoutObject>>) -> Self {
        self.layout_object = layout_object.into();
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