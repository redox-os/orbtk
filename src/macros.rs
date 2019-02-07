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
        use crate::widget::{SharedProperty, Template};

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
            fn $shared_method(self, $method: SharedProperty) -> Self {
                self.template(|template| template.shared_property($method.into()))
            }
        }
    };
}