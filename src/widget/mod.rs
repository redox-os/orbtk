//! This module contains the base structures for widget creation and concret implementations of OrbTk's default widgets. It contains also layout widgets.

use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use dces::{Component, ComponentBox, Entity, EntityComponentManager, NotFound, SharedComponentBox};

use application::Tree;
use enums::ParentType;
use event::EventHandler;
use layout_object::{LayoutObject, RootLayoutObject};
use render_object::RenderObject;
use theme::{Selector, Theme};

pub use self::button::Button;
pub use self::center::Center;
pub use self::check_box::CheckBox;
pub use self::column::Column;
pub use self::container::Container;
pub use self::cursor::Cursor;
pub use self::font_icon_block::FontIconBlock;
pub use self::row::Row;
pub use self::scroll_viewer::*;
pub use self::spacer::*;
pub use self::stack::Stack;
pub use self::text_block::TextBlock;
pub use self::text_box::*;
pub use self::toggle_button::ToggleButton;
pub use self::water_mark_text_block::WaterMarkTextBlock;

mod button;
mod center;
mod check_box;
mod column;
mod container;
mod cursor;
mod font_icon_block;
mod row;
mod scroll_viewer;
mod spacer;
mod stack;
mod text_block;
mod text_box;
mod toggle_button;
mod water_mark_text_block;

/// The `PropertyResult` enum is used to create concrete shared properties for a widget on run time from `SharedProperty` struct.
pub enum PropertyResult {
    Property(ComponentBox, Rc<Cell<Option<Entity>>>),
    Source(SharedComponentBox),
    PropertyNotFound,
}

/// The `SharedProperty` struct is used to define shared properties for widgets. A shared property could be shared between different widgets.
/// All refernces of a shared property will always share the same value. Only the origin shared property contains the concret property, all
/// other cloned shared properties only references to the origin.
pub struct SharedProperty {
    pub source_chain: Rc<RefCell<Vec<Rc<Cell<Option<Entity>>>>>>,
    property: Option<ComponentBox>,
    type_id: TypeId,
}

impl SharedProperty {
    /// Creates an new `SharedProperty` for the given `property`.
    pub fn new<P: Component>(property: P) -> Self {
        SharedProperty {
            source_chain: Rc::new(RefCell::new(vec![Rc::new(Cell::new(None))])),
            property: Some(ComponentBox::new::<P>(property)),
            type_id: TypeId::of::<P>(),
        }
    }

    // Use to change the inner `property` of the origin.
    fn update_property<P: Component>(&mut self, property: P) {
        self.property = Some(ComponentBox::new(property));
    }

    /// Returns the concret property if the shared property is orgin. If the shared property contains a refernce to its origin the method returns
    /// a `SharedComponentBox`. If its not the origion and does not contain a reference to the origin `PropertyResult::PropertyNotFound` will be returned.
    pub fn build(self) -> PropertyResult {
        if let Some(property) = self.property {
            return PropertyResult::Property(property, self.source_chain.borrow()[0].clone());
        }

        if let Some(source) = self.source_chain.borrow()[self.source_chain.borrow().len() - 1].get()
        {
            return PropertyResult::Source(SharedComponentBox::new(self.type_id, source));
        }

        PropertyResult::PropertyNotFound
    }
}

impl Clone for SharedProperty {
    fn clone(&self) -> Self {
        SharedProperty {
            source_chain: self.source_chain.clone(),
            property: None,
            type_id: self.type_id,
        }
    }
}

/// The `WidgetKey` struct is used to get access of a widget by using a string.
pub struct WidgetKey {
    /// The `key` string is used to call a widget with a string.
    key: String,

    /// The `entity` will generated on runtime and is used to access the widget.
    pub entity: Rc<Cell<Option<Entity>>>,
}

impl From<&str> for WidgetKey {
    fn from(s: &str) -> WidgetKey {
        WidgetKey {
            key: s.to_string(),
            entity: Rc::new(Cell::new(None)),
        }
    }
}

impl From<String> for WidgetKey {
    fn from(s: String) -> WidgetKey {
        WidgetKey {
            key: s,
            entity: Rc::new(Cell::new(None)),
        }
    }
}

impl Clone for WidgetKey {
    fn clone(&self) -> Self {
        WidgetKey {
            key: self.key.clone(),
            entity: self.entity.clone(),
        }
    }
}

/// The `KeyChain` structs provides all widget children keys.
pub struct KeyChain {
    key_chain: HashMap<String, WidgetKey>,
}

impl Default for KeyChain {
    fn default() -> Self {
        KeyChain {
            key_chain: HashMap::new(),
        }
    }
}

impl KeyChain {
    /// If the the key chain contains the requested key it will return the entity of the requested widget, otherweise it will return `None`.
    pub fn get(&self, key: &str) -> Option<Entity> {
        if let Some(key) = self.key_chain.get(key) {
            return key.entity.get();
        } else {
            None
        }
    }

    /// Registers a new key in the key chain.
    pub fn register_key(&mut self, key: WidgetKey) {
        self.key_chain.insert(key.key.clone(), key);
    }
}

/// The `Context` is provides acces for the states to objects they could work with.
pub struct Context<'a> {
    pub widget: &'a mut WidgetContainer<'a>,
    pub theme: &'a Theme,
}

/// Used to define a state of a widget. A state is used to customize properties of a widget.
pub trait State {
    /// Updates the state for the given `context`.
    fn update(&self, context: &mut Context);

    /// This update method is called after layout is calculated and before rendering.
    fn update_post_layout(&self, _context: &mut Context) {}
}

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
    pub key: Option<WidgetKey>,
    pub key_chain: Option<KeyChain>,
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
            key: None,
            key_chain: None,
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

    /// Registers a key for the template.
    pub fn with_key(mut self, key: WidgetKey) -> Self {
        self.key = Some(key);
        self
    }

    /// Adds a new child `WidgetKey` to the `KeyChain` of the widget.
    pub fn with_child_key(mut self, key: WidgetKey) -> Self {
        if let None = self.key_chain {
            self.key_chain = Some(KeyChain::default());
        }

        if let Some(key_chain) = &mut self.key_chain {
            key_chain.register_key(key);
        }

        self
    }
}

/// The `Widget` trait is used to define a new widget.
pub trait Widget {
    /// Returns the template of the implemented widget.
    fn create() -> Template;
}

/// The `WidgetContainer` wraps the entity of a widget and provides access to its propeties, its children properties and its parent properties.
pub struct WidgetContainer<'a> {
    tree: &'a Tree,
    ecm: &'a mut EntityComponentManager,
    current_node: Entity,
}

impl<'a> WidgetContainer<'a> {
    /// Creates a new widget container for the given `entity`.
    pub fn new(root: Entity, ecm: &'a mut EntityComponentManager, tree: &'a Tree) -> Self {
        WidgetContainer {
            tree,
            ecm,
            current_node: root,
        }
    }

    /// Returns a reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it dosen't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_property<P: Component>(&self) -> Result<&P, NotFound> {
        self.ecm.borrow_component::<P>(self.current_node)
    }

    /// Returns a mutable reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it dosen't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_mut_property<P: Component>(&mut self) -> Result<&mut P, NotFound> {
        self.ecm.borrow_mut_component::<P>(self.current_node)
    }

    /// Returns a reference of a property of type `P` from the given widget entity by `key`. If the entity does
    /// not exists or it dosen't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_property_by_key<P: Component>(&self, key: &str) -> Result<&P, NotFound> {
        if let Ok(key_chain) = self.borrow_property::<KeyChain>() {
            if let Some(entity) = key_chain.get(key) {
                return self.ecm.borrow_component::<P>(entity);
            }
        }

        Result::Err(NotFound::Unkown(format!("Key not found: {}", key)))
    }

    /// Returns a mutable reference of a property of type `P` from the given widget entity by `key`. If the entity does
    /// not exists or it dosen't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_mut_property_by_key<P: Component>(
        &mut self,
        key: &str,
    ) -> Result<&mut P, NotFound> {
        let mut entity = None;

        if let Ok(key_chain) = self.borrow_property::<KeyChain>() {
            if let Some(en) = key_chain.get(key) {
                entity = Some(en);
            }
        }

        if let Some(entity) = entity {
            return self.ecm.borrow_mut_component::<P>(entity);
        }

        Result::Err(NotFound::Unkown(format!("Key not found: {}", key)))
    }

    /// Returns a reference of a property of type `P` from the given widgets parent entity. If the entity does
    /// not exists or it dosen't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_parent_property<P: Component>(&self) -> Result<&P, NotFound> {
        self.ecm
            .borrow_component::<P>(self.tree.parent[&self.current_node])
    }

    /// Returns a mutable reference of a property of type `P` from the given widgets parent entity. If the entity does
    /// not exists or it dosen't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_mut_parent_property<P: Component>(&mut self) -> Result<&mut P, NotFound> {
        self.ecm
            .borrow_mut_component::<P>(self.tree.parent[&self.current_node])
    }

    /// Returns a reference of a property of type `P` from the given widgets child entity. If the entity does
    /// not exists or it dosen't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_child_property<P: Component>(&self, index: usize) -> Result<&P, NotFound> {
        if index >= self.tree.children[&self.current_node].len() {
            return Result::Err(NotFound::Component(TypeId::of::<P>()));
        }

        self.ecm
            .borrow_component::<P>(self.tree.children[&self.current_node][index])
    }

    /// Returns a mutable reference of a property of type `P` from the given widgets child entity. If the entity does
    /// not exists or it dosen't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_mut_child_property<P: Component>(
        &mut self,
        index: usize,
    ) -> Result<&mut P, NotFound> {
        if index >= self.tree.children[&self.current_node].len() {
            return Result::Err(NotFound::Component(TypeId::of::<P>()));
        }

        self.ecm
            .borrow_mut_component::<P>(self.tree.children[&self.current_node][index])
    }
}

/// Adds the given `pseudo_class` to the css selector of the given `wiget`.
pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.insert(String::from(pseudo_class));
    }
}

/// Removes the given `pseudo_class` from the css selector of the given `wiget`.
pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.remove(pseudo_class);
    }
}
