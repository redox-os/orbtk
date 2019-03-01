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

#[macro_export]
macro_rules! wip_property {
    ($(#[$widget_doc:meta])* $property:ident: $type:ident($(#[$name_doc:meta])* $name:ident, $(#[$prop_name_doc:meta])* $prop_name:ident)) => {
        use dces::prelude::{Entity, EntityComponentManager};

        use crate::widget::{get_property, Property, Template};

        $(#[$widget_doc])*
        pub trait $property: Sized + From<Template> + Into<Template> {
            /// Transforms the property into a template.
            fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
                Self::from(transform(self.into()))
            }

            $(#[$name_doc])*
            fn $name<V: Into<$type>>(self, $name: V) -> Self {
                self.$prop_name($name.into())
            }

            $(#[$prop_name_doc])*
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