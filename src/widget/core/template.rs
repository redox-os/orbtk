use std::{any::TypeId, collections::HashMap, rc::Rc};

use dces::prelude::{Component, ComponentBox};

use crate::{
    enums::ParentType,
    event::EventHandler,
    layout::{GridLayout, Layout},
    properties::{Bounds, Constraint, HorizontalAlignment, VerticalAlignment, Visibility},
    render_object::RenderObject,
    structs::Point,
};

use super::{SharedProperty, State};

/// `Template` is used to define the inner structure of a widget.
///
/// Intern it is used to create an entity with components for the widget.
///
/// # Properties of each widget
///
/// * Bounds - describes the bounds of the widget.
/// * Constraint - box constraints with (min | max) width and height.
/// * Margin - define space around a widget.
/// * VerticalAlignment - align widget on the vertical axis.
/// * HorizontalAlignment - align widget on the horizontal axis.
///
/// # CSS properties of each widget
///
/// * margin
/// * (min | max) width
/// * (min | max) height
///
/// # Others
///
/// * `ParentType`- Single (default).
pub struct Template {
    pub children: Vec<Template>,
    pub parent_type: ParentType,
    pub state: Option<Rc<dyn State>>,
    pub event_handlers: Vec<Rc<dyn EventHandler>>,
    pub render_object: Option<Box<dyn RenderObject>>,
    pub layout: Box<dyn Layout>,
    pub constraint: Constraint,
    pub properties: HashMap<TypeId, ComponentBox>,
    pub shared_properties: HashMap<TypeId, SharedProperty>,
    pub debug_name: String,
}

impl Default for Template {
    fn default() -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            TypeId::of::<Bounds>(),
            ComponentBox::new::<Bounds>(Bounds::default()),
        );
        properties.insert(
            TypeId::of::<Constraint>(),
            ComponentBox::new::<Constraint>(Constraint::default()),
        );
        properties.insert(
            TypeId::of::<VerticalAlignment>(),
            ComponentBox::new::<VerticalAlignment>(VerticalAlignment::default()),
        );
        properties.insert(
            TypeId::of::<HorizontalAlignment>(),
            ComponentBox::new::<HorizontalAlignment>(HorizontalAlignment::default()),
        );
        properties.insert(
            TypeId::of::<Visibility>(),
            ComponentBox::new::<Visibility>(Visibility::default()),
        );

        properties.insert(
            TypeId::of::<Point>(),
            ComponentBox::new::<Point>(Point::new(0.0, 0.0)),
        );

        Template {
            children: vec![],
            parent_type: ParentType::Single,
            state: None,
            event_handlers: vec![],
            render_object: None,
            layout: Box::new(GridLayout::default()),
            constraint: Constraint::new(),
            properties,
            shared_properties: HashMap::new(),
            debug_name: String::default(),
        }
    }
}

impl Template {
    /// Creates a new template with default values.
    pub fn new() -> Self {
        Template::default()
    }
    /// Set the debug name of the widget. It is used to print the name of the widget while widget creation if `debug_flag` on window is set to `true`.
    pub fn debug_name(mut self, name: impl Into<String>) -> Self {
        self.debug_name = name.into();
        self
    }

    /// Define the `parent_type` of a widget. The `parent_type` should be set before start adding children.
    pub fn parent_type(mut self, parent_type: ParentType) -> Self {
        self.parent_type = parent_type;
        self
    }

    /// Add a child to the widget template. If `parent_type` is set to `ParentType::None` this method do nothing. If `parent_type` is set to `ParentType::Single` only on child could
    /// be added. Every call of this method will overwrite the existing `child`. If `parent_type`is set to `ParentType::Multiple` any number of children could be added to the template.
    pub fn child<T: Into<Template>>(mut self, child: T) -> Self {
        match self.parent_type {
            ParentType::Single => {
                self.children.clear();
                self.children.push(child.into());
            }
            ParentType::Multi => {
                self.children.push(child.into());
            }
            _ => return self,
        }

        self
    }

    /// Used to add a `state' to the template. Only one `state` can be added.
    pub fn state(mut self, state: Rc<dyn State>) -> Self {
        self.state = Some(state);
        self
    }

    /// Add an event handler to the template. Multiple event handlers can be added.
    pub fn event_handler(mut self, handler: impl Into<Rc<dyn EventHandler>>) -> Self {
        self.event_handlers.push(handler.into());
        self
    }

    /// Used to add a `render_object' to the template. Only one `render_object` can be added.
    pub fn render_object(mut self, render_object: impl Into<Box<dyn RenderObject>>) -> Self {
        self.render_object = Some(render_object.into());
        self
    }

    /// Used to add a `layout' to the template. Only one `layout` can be added.
    pub fn layout(mut self, layout: impl Into<Box<dyn Layout>>) -> Self {
        self.layout = layout.into();
        self
    }

    /// Overwrite the constraint of the template.
    pub fn constraint<C: Into<Constraint>>(mut self, constraint: C) -> Self {
        self.constraint = constraint.into();
        self
    }

    /// Inserts a new width.
    pub fn width(mut self, width: f64) -> Self {
        self.constraint.set_width(width);
        self
    }

    /// Inserts a new height.
    pub fn height(mut self, height: f64) -> Self {
        self.constraint.set_height(height);
        self
    }

    /// Inserts a new size.
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.constraint.set_width(width);
        self.constraint.set_height(height);
        self
    }

    /// Inserts a new min_width.
    pub fn min_width(mut self, min_width: f64) -> Self {
        self.constraint.set_min_width(min_width);
        self
    }

    /// Inserts a new min_height.
    pub fn min_height(mut self, min_height: f64) -> Self {
        self.constraint.set_min_height(min_height);
        self
    }

    /// Inserts a new min_size.
    pub fn min_size(mut self, min_width: f64, min_height: f64) -> Self {
        self.constraint.set_min_width(min_width);
        self.constraint.set_min_height(min_height);
        self
    }

    /// Inserts a new max_width.
    pub fn max_width(mut self, max_width: f64) -> Self {
        self.constraint.set_max_width(max_width);
        self
    }

    /// Inserts a new max_height.
    pub fn max_height(mut self, max_height: f64) -> Self {
        self.constraint.set_max_height(max_height);
        self
    }

    /// Inserts a new min_size.
    pub fn max_size(mut self, max_width: f64, max_height: f64) -> Self {
        self.constraint.set_max_width(max_width);
        self.constraint.set_max_height(max_height);
        self
    }

    /// Used to register a `property' for the template. Only one property per type can be registered.
    /// If a shared property of the same type exists the value of the shared property will be set to
    /// the given property's value.
    pub fn property<C: Component>(mut self, property: C) -> Self {
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
    pub fn shared_property(mut self, property: SharedProperty) -> Self {
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

    /// Inserts the vertical alignment.
    pub fn vertical_alignment<V: Into<VerticalAlignment>>(self, vertical_alignment: V) -> Self {
        self.property(vertical_alignment.into())
    }

    /// Inserts a shared vertical alignment.
    pub fn shared_vertical_alignment<V: Into<VerticalAlignment>>(
        self,
        vertical_alignment: V,
    ) -> Self {
        self.shared_property(SharedProperty::new(vertical_alignment.into()))
    }
}

/// Used as base for widget templates.
pub trait TemplateBase: Sized + From<Template> + Into<Template> {
    /// Transforms the widget into a template.
    fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
        Self::from(transform(self.into()))
    }

    /// Inserts a debug name.
    fn debug_name(self, name: impl Into<String>) -> Self {
        self.template(|template| template.debug_name(name))
    }

    /// Inserts a parent type.
    fn parent_type(self, parent_type: ParentType) -> Self {
        self.template(|template| template.parent_type(parent_type))
    }

    /// Inserts a child.
    fn child<T: Into<Template>>(self, child: T) -> Self {
        self.template(|template| template.child(child))
    }

    /// Inserts a state.
    fn state(self, state: Rc<dyn State>) -> Self {
        self.template(|template| template.state(state))
    }

    /// Inserts a event handler.
    fn event_handler(self, handler: impl Into<Rc<dyn EventHandler>>) -> Self {
        self.template(|template| template.event_handler(handler))
    }

    /// Inserts a render object.
    fn render_object(self, render_object: impl Into<Box<dyn RenderObject>>) -> Self {
        self.template(|template| template.render_object(render_object))
    }

    /// Inserts a layout.
    fn layout(self, layout: impl Into<Box<dyn Layout>>) -> Self {
        self.template(|template| template.layout(layout))
    }

    /// Overwrite the constraint of the template.
    fn constraint<C: Into<Constraint>>(self, constraint: C) -> Self {
        self.template(|template| template.constraint(constraint))
    }

    /// Inserts a new width.
    fn width(self, width: f64) -> Self {
        self.template(|template| template.width(width))
    }

    /// Inserts a new height.
    fn height(self, height: f64) -> Self {
        self.template(|template| template.height(height))
    }

    /// Inserts a new size.
    fn size(self, width: f64, height: f64) -> Self {
        self.template(|template| template.size(width, height))
    }

    /// Inserts a new min_width.
    fn min_width(self, min_width: f64) -> Self {
        self.template(|template| template.min_width(min_width))
    }

    /// Inserts a new min_height.
    fn min_height(self, min_height: f64) -> Self {
        self.template(|template| template.min_height(min_height))
    }

    /// Inserts a new min_size.
    fn min_size(self, min_width: f64, min_height: f64) -> Self {
        self.template(|template| template.min_size(min_width, min_height))
    }

    /// Inserts a new max_width.
    fn max_width(self, max_width: f64) -> Self {
        self.template(|template| template.max_width(max_width))
    }

    /// Inserts a new max_height.
    fn max_height(self, max_height: f64) -> Self {
        self.template(|template| template.max_height(max_height))
    }

    /// Inserts a new min_size.
    fn max_size(self, max_width: f64, max_height: f64) -> Self {
        self.template(|template| template.max_size(max_width, max_height))
    }

    /// Attaches a property.
    fn attach_property<C: Component>(self, property: C) -> Self {
        self.template(|template| template.property(property))
    }

    /// Attaches a shared property.
    fn attach_shared_property(self, property: SharedProperty) -> Self {
        self.template(|template| template.shared_property(property))
    }
}
