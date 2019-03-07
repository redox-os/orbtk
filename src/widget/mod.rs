//! This module contains the base structures for widget creation and concrete implementations of OrbTk 's default widgets. It contains also layout widgets.

pub use self::button::*;
pub use self::core::*;
// pub use self::canvas_widget::CanvasWidget;
pub use self::check_box::*;
pub use self::container::*;
pub use self::cursor::*;
pub use self::font_icon_block::*;
pub use self::grid::*;
pub use self::image_widget::*;
pub use self::scroll_viewer::ScrollViewer;
pub use self::stack::*;
pub use self::switch::*;
pub use self::text_block::*;
pub use self::text_box::*;
pub use self::toggle_button::*;
pub use self::water_mark_text_block::*;

mod button;
mod core;
// mod canvas_widget;
mod check_box;
mod container;
mod cursor;
mod font_icon_block;
mod grid;
mod image_widget;
mod scroll_viewer;
mod stack;
mod switch;
mod text_block;
mod text_box;
mod toggle_button;
mod water_mark_text_block;

use dces::prelude::*;
use crate::application::Tree;

pub struct WipBuildContext<'a> {
    world: &'a mut World<Tree>,
}

impl<'a> WipBuildContext<'a> {
    pub fn new(world: &'a mut World<Tree>) -> Self {
        WipBuildContext {
            world,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.world.create_entity().build()
    }

    pub fn register_property<P: Component>(&mut self, entity: Entity, property: P) {
        self.world.entity_component_manager().register_component(entity, property);
    }

    pub fn register_shared_property<P:Component>(&mut self, target: Entity, source: Entity) {
        self.world.entity_component_manager().register_shared_component::<P>(target, source);
    }    
}

pub struct WipTemplate {
    id: Entity,
    children: Vec<WipTemplate>,
    // todo parent type
}

impl WipTemplate {
    pub fn new(id: Entity) -> Self {
        WipTemplate {
            id,
            children: vec![],
        }
    }

    pub fn child(mut self, template: WipTemplate) -> Self {
        self.children.push(template);
        self
    }
}

pub struct WipButton<'a> {
    id: Entity,
    context: &'a mut WipBuildContext<'a>, 
}

wip_property!(WipBackground(String));

impl From<&str> for WipBackground {
    fn from(s: &str) -> WipBackground {
        WipBackground(s.into())
    }
}

wip_property!(WipForeground(String));

impl From<&str> for WipForeground {
    fn from(s: &str) -> WipForeground {
        WipForeground(s.into())
    }
}

pub trait WipProperty<'a, P> where P: Component {
    type Widget;

    fn value(self, value: impl Into<P>) -> Self::Widget;
    fn source(self, id: Entity) -> Self::Widget;
}

mod inner {
    use std::marker::PhantomData;
    use super::{WipButton, WipProperty};
    use dces::prelude::{Component, Entity};

    pub struct Property<'a, P> where P: Component {
        widget: WipButton<'a>,
        property_type: PhantomData<P>, 
    }

     impl<'a, P: 'static> Property <'a, P> {
         pub fn new(widget: WipButton<'a>) -> Property<'a, P> {
             Property {
                 widget,
                 property_type: PhantomData,
             }
         }     
     }

    impl<'a, P: 'static> WipProperty<'a, P> for Property<'a, P> {
        type Widget = WipButton<'a>;

         fn value(self, value: impl Into<P>) -> Self::Widget {
             self.widget.context.register_property(self.widget.id, value.into());
             self.widget
         }

         fn source(self, source: Entity) -> Self::Widget {
             self.widget.context.register_shared_property::<P>(self.widget.id, source);
             self.widget
         }
    }    
}

pub trait WipWidget<'a> {
    fn create(context: &'a mut WipBuildContext<'a>) -> Self;
    fn build(self) -> WipTemplate;
}

impl<'a> WipButton<'a> {
    pub fn background(self) -> inner::Property<'a, WipBackground>{
        inner::Property::new(self)
    }

    pub fn foreground(self) -> inner::Property<'a, WipForeground>{
        inner::Property::new(self)
    }
}

impl<'a> WipWidget<'a> for WipButton<'a> {
     fn create(context: &'a mut WipBuildContext<'a>) -> Self {
        WipButton {
            id: context.create_entity(),
            context
        }
    }

    fn build(self) -> WipTemplate {
        WipTemplate::new(self.id)

    }
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