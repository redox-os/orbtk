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

// macro_rules! wip_property {
//     ($property:ident, $value:ident) => {
//         use std::rc::Rc;
//         use std::cell::{ Cell, RefCell };

//         // use dces::entity::Entity;
//         use dces::prelude::ComponentBox;

//         use crate::widget::{ WipProperty, WipPropertyBuilder };

//         pub enum $property {
//             Property($value),
//             Builder{ value: Option<$value>, source_chain: Rc<RefCell<Vec<Rc<Cell<Option<Entity>>>>>> }
//         }

//         impl WipProperty for $property {
//             type Value = $value;

//             fn value(&self) -> Self::Value {
//                 match self {
//                     $property::Property(value) => return value.clone(),
//                     _ => return $value::default()
//                 }
//             }

//             fn set_value(&mut self, value: Self::Value) {
//                 *self = $property::Property(value);
//             }
//         }

//         impl WipPropertyBuilder for $property {
//             type Value = $value;

//             fn build(self) -> (Option<ComponentBox>, Option<Rc<RefCell<Vec<Rc<Cell<Option<Entity>>>>>>>) {
//                 match self {
//                     $property::Builder { value, source_chain } => {
//                         if let Some(value) = value {
//                             return (Some(ComponentBox::new($property::Property(value))), Some(source_chain))
//                         } else {
//                             return (None, Some(source_chain));
//                         }
//                     },
//                     _ => (None, None)
//                 }
//             }
//         }
//     };
// }

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

macro_rules! wip_widget {
    ( $(#[$widget_doc:meta])* $widget:ident { $($(#[$prop_doc:meta])* $property:ident: $property_type:tt ),*} ) => {
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

                pub fn child(mut self, child: Template) -> Self {
                    self.template = template.child(child);
                    self
                }
            }

             $(
                $(#[$prop_doc])*
                pub fn $property(mut self, $property: impl Into<$Property>) -> Self {
                    self.template.property($property.into());
                    self
                }
            )*
    };
}

/*

    pub struct Button {
        build_context: &mut BuildContext,
        id: Entity,
    }

    pub struct BuildContext {
        world: &mut World,
    }

    impl BuildContext {
        fn property() -> Entity,
        ...
    }

    pub fn new(build_context: &mut BuildContext) -> Self {
        let id = build_context.create_widget();

        Button {
            build_context,
            id,
        }

    }
    Button::create(build_context).background().value("green").foreground().value("blue").attach(GridColumn(0)).attach_source::<GridRow>::(self.id).build()

    pub struct Property {
        widget: Button,
    }

     impl Property<P: Component> {
         pub fn value(self, value: impl Into<P>) -> Button {
             self.widget
         }

         pub fn source(id: Entity) -> Button {

         }
     }

    fn background(self) -> Property {
        Property {
            widget: self,
        }
    }

    fn attach<P: Component>(property: impl Into<C>) -> Self {

    }



    fn build(mut self) -> Template {
        Template::new()
            .child(Container::new(self.build_context).background().source(self.id))
    }
*/

#[macro_export]
macro_rules! wip_property {
    ($property:ident($type:ty)) => {
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
    };
}