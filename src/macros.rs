/// Used to define a widget, with properties and event handlers.
#[macro_export]
macro_rules! widget {
    ( $(#[$widget_doc:meta])* $type:ident ) => (
        widget!( $type () );
    );
    
    ( $(#[$widget_doc:meta])* $type:ident ( $( $derive:ident ),* ] ) => (
        use crate::{
            widget::TemplateBase,
            properties::{
                HorizontalAlignmentProperty,
                VerticalAlignmentProperty,
                EnabledProperty,
                VisibilityProperty,
                MarginProperty,
            },
            theme::SelectorProperty,
        };

        $(#[$widget_doc])*
        pub struct $type {
            template: Template
        }

        impl $type {
            /// Creates a new widget.
            pub fn new() -> Self {
                $type {
                    template: Template::new()
                }
            }
        }

        impl From<Template> for $type {
            fn from(template: Template) -> Self {
                $type {
                    template
                }
            }
        }

        impl Into<Template> for $type {
            fn into(self) -> Template {
                self.template
            }
        }

        impl TemplateBase for $type {}

        impl HorizontalAlignmentProperty for $type {}

        impl VerticalAlignmentProperty for $type {}

        impl SelectorProperty for $type {}

        impl EnabledProperty for $type {}

        impl VisibilityProperty for $type {}

        impl MarginProperty for $type {}

        $(
            impl $derive for $type {}
        )*
    )
}

/// Used to define a property.
#[macro_export]
macro_rules! property {
    ($(#[$widget_doc:meta])* $type:ident, $property:ident, $name:ident, $prop_name:ident) => {
        use dces::prelude::{Entity, EntityComponentManager};

        use crate::widget::{get_property, Property, Template};

        $(#[$widget_doc])*
        pub trait $property: Sized + From<Template> + Into<Template> {
            /// Transforms the property into a template.
            fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
                Self::from(transform(self.into()))
            }

            /// Inserts a property.
            fn $name<V: Into<$type>>(self, $name: V) -> Self {
                self.$prop_name($name.into())
            }

            /// Inserts a shared property.
            fn $prop_name(self, $name: impl Into<Property>) -> Self {
                self.template(|template| template.property($name.into())
            }
        }

        impl From<$type> for Property {
            fn from(prop: $type) -> Self {
                Property::new(prop)
            }
        }

        impl $type {
            /// Returns the value of a property.
            pub fn get(entity: Entity, ecm: &EntityComponentManager) -> $type {
                get_property::<$type>(entity, ecm)
            }
        }
    };
}

/// Used to define a property.
#[macro_export]
macro_rules! wip_property {
    ($property:ident($type:ty)) => {
        #[derive(Default)]
        pub struct $property(pub $type);

        impl From<$property> for $type {
            fn from(property: $property) -> $type {
                property.0.into()
            }
        } 

        impl From<$type> for $property {
            fn from(value: $type) -> $property {
                $property(value)
            }
        } 

        impl Into<PropertySource<$property>> for $type {
            fn into(self) -> PropertySource<$property> {
                PropertySource::Value($property::from(self))
            }
        }

        impl Into<PropertySource<$property>> for Entity {
            fn into(self) -> PropertySource<$property> {
                PropertySource::Source(self)
            }
        }
    };
}

/// Used to define a widget, with properties and event handlers.
macro_rules! wip_widget {
    ( $(#[$widget_doc:meta])* $widget:ident $(: $( $handler:ident ),*)* { $($(#[$prop_doc:meta])* $property:ident: $property_type:tt ),* } ) => {
        use std::{ any::TypeId, rc::Rc, collections::HashMap};

        use dces::prelude::{Component, ComponentBox, SharedComponentBox };

        use crate::{event::EventHandler, widget::{PropertySource, WipWidget}};

        $(#[$widget_doc])*
        pub struct $widget {
            attached_properties: HashMap<TypeId, ComponentBox>,
            shared_attached_properties: HashMap<TypeId, SharedComponentBox>,
            event_handlers: Vec<Rc<dyn EventHandler>>,
            $(
                $property: PropertySource<$property_type>,
            )*
            children: Vec<Entity>,
        }

        impl $widget {
            /// Sets an attached property or shares it by the given id.
            pub fn attach<P: Component>(mut self, property: impl Into<PropertySource<P>>) -> Self {
                match property.into() {
                    PropertySource::Value(value) => {
                        self.attached_properties.insert(TypeId::of::<P>(), ComponentBox::new(value));
                    },
                    PropertySource::Source(source) => {
                        self.shared_attached_properties.insert(TypeId::of::<P>(), SharedComponentBox::new(TypeId::of::<P>(), source));
                    }
                }
                self
            }
            $(
                $(#[$prop_doc])*
                pub fn $property<P: Into<PropertySource<$property_type>>>(mut self, $property: P) -> Self {
                    self.$property = $property.into();
                    self
                }
            )*
        }

        $(
            $(
                impl $handler for $widget {}
            )*
        )*

        impl WipWidget for $widget {
            fn create() -> Self {
                $widget {
                    attached_properties: HashMap::new(),
                    shared_attached_properties: HashMap::new(),
                    event_handlers: vec![],
                    $(
                        $property: PropertySource::Value($property_type::default()),
                    )*
                    children: vec![],
                }
            }

            fn insert_handler(mut self, handler: impl Into<Rc<dyn EventHandler>>) -> Self {
                self.event_handlers.push(handler.into());
                self
            }

            fn child(mut self, child: Entity) -> Self {
                self.children.push(child);
                self
            }

            fn build(self, context: &mut WipBuildContext) -> Entity {
                let entity = context.create_entity();

                for (_, property) in self.attached_properties {
                    context.register_property_box(entity, property);
                }

                for (_, property) in self.shared_attached_properties {
                    context.register_property_shared_box(entity, property);
                }

                $(
                    match self.$property {
                        PropertySource::Value(value) => {
                            context.register_property(entity, value);
                        },
                        PropertySource::Source(source) => {
                            context.register_shared_property::<$property_type>(entity, source);
                        }
                    }
                )*

                for child in self.children {
                    context.append_child(entity, child);
                }

                entity
                // $widget::template(context.create_entity(), context)
            }
        }
    };
}