// #[macro_export]
// macro_rules!wip_widget {
//      ( $(#[$widget_doc:meta])* $type:ident ) => (
//         wip_widget!($type { properties {}, handlers {} });
//      );

//     ( $(#[$widget_doc:meta])* $type:ident { properties{ $( $prop_name:ident: $prop_type:ty ),* } } ) => (
//         wip_widget!($type { properties { $( $prop_name: $prop_type )* }, handlers {} } );
//     );

//     ( $(#[$widget_doc:meta])* $type:ident { properties { $( $prop_name:ident: $prop_type:ty ),* },  handlers { $( $hand_name:ident: hand_type:ty ),* } } ) => (
//         use crate::{
//             widget::{ TemplateBase, Template },
//             properties::{
//                 HorizontalAlignmentProperty,
//                 VerticalAlignmentProperty,
//                 EnabledProperty,
//                 VisibilityProperty,
//                 MarginProperty,
//             },
//             theme::SelectorProperty,
//         };

//         $(#[$widget_doc])*
//         pub struct $type {
//             template: Template
//         }

//         impl $type {
//             /// Creates a new widget.
//             pub fn new() -> Self {
//                 $type {
//                     template: Template::new()
//                 }
//             }

//             $(
//                 fn $prop_name<V: Into<$prop_type>>(self, $prop_name: V) -> Self {
//                     self.$shared_method($prop_name.into())
//                 }

//                 fn $prop_name_shared(self, $prop_name: impl Into<Property>) -> Self {
//                     self.template(|template| template.property($prop_name.into())
//                 }
//             )*
//         }

//         impl From<Template> for $type {
//             fn from(template: Template) -> Self {
//                 $type {
//                     template
//                 }
//             }
//         }

//         impl Into<Template> for $type {
//             fn into(self) -> Template {
//                 self.template
//             }
//         }

//         // impl TemplateBase for $type {}

//         // impl HorizontalAlignmentProperty for $type {}

//         // impl VerticalAlignmentProperty for $type {}

//         // impl SelectorProperty for $type {}

//         // impl EnabledProperty for $type {}

//         // impl VisibilityProperty for $type {}

//         // impl MarginProperty for $type {}

//         // $(
//         //     impl $derive for $type {}
//         // )*
//     )
// }


/// Used to define a widget, with properties and event handlers.
#[macro_export]
macro_rules! widget {
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
                self.$shared_method($method.into())
            }

            /// Inserts a shared property.
            fn $shared_method(self, $method: impl Into<Property>) -> Self {
                self.template(|template| template.property($method.into())
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