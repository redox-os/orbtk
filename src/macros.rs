/// Used to define a widget template, with properties and event handlers.
#[macro_export]
macro_rules! template {
    ($type:ident, [ $( $derive:ident ),* ]) => (

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

        pub struct $type {
            template: Template
        }

        impl $type {
            /// Creates a new template.
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
    ($type:ident, $property:ident, $method:ident, $shared_method:ident) => {
        use dces::prelude::{Entity, EntityComponentManager};

        use crate::widget::{get_property, Property, Template};

        pub trait $property: Sized + From<Template> + Into<Template> {
            /// Transforms the property into a template.
            fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
                Self::from(transform(self.into()))
            }

            /// Inserts a property.
            fn $method<V: Into<$type>>(self, $method: V) -> Self {
                self.template(|template| template.property($method.into()))
            }

            /// Inserts a shared property.
            fn $shared_method(self, $method: Property) -> Self {
                self.template(|template| template.shared_property($method.into()))
            }
        }

        impl $type {
            pub fn get(entity: Entity, ecm: &EntityComponentManager) -> $type {
                get_property::<$type>(entity, ecm)
            }
        }
    };
}

macro_rules! widget {
    ( $(#[$widget_doc:meta])* $widget:ident { $($(#[$prop_doc:meta])* $property:ident: $property_type:tt ),*} ) => {
        $(#[$widget_doc])*
        pub struct $widget {
            $(
                $property: $property_type,
            )*
        }

        impl $widget {
            /// Creates a new instance of the widget, with all its properties. Used to build the template of the widget.
            pub fn create() -> Self {
               $widget {
                    $(
                        $property: $property_type::default(),
                    )*
               }
            }

            $(
                $(#[$prop_doc])*
                pub fn $property(mut self, $property: impl Into<$property_type>) -> Self {
                    self.$property = $property.into();
                    self
                }
            )*

            /// Builds the template of the widget.
            pub fn build(self) -> Template {
                let mut template = self.template();

                $(
                    template.insert_property(TypeId::of::<$property_type>(), self.$property.into());
                )*

                template
            }
        }
    };
}