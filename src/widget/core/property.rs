use std::{
    any::TypeId,
    cell::{Cell, RefCell},
    rc::Rc,
};

use dces::prelude::{Component, ComponentBox, Entity, SharedComponentBox};

/// The `PropertyResult` enum is used to create concrete shared properties for a widget on run time from `SharedProperty` struct.
pub enum PropertyResult {
    Property(ComponentBox, Rc<Cell<Option<Entity>>>),
    Source(SharedComponentBox),
    PropertyNotFound,
}

/// The `SharedProperty` struct is used to define shared properties for widgets. A shared property could be shared between different widgets.
/// All references of a shared property will always share the same value. Only the origin shared property contains the concert property, all
/// other cloned shared properties only references to the origin.
pub struct SharedProperty {
    pub source_chain: Rc<RefCell<Vec<Rc<Cell<Option<Entity>>>>>>,
    pub property: Option<ComponentBox>,
    pub type_id: TypeId,
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

    /// Use to change the inner `property` of the origin.
    pub fn update_property<P: Component>(&mut self, property: P) {
        self.property = Some(ComponentBox::new(property));
    }

    /// Returns the concert property if the shared property is origin. If the shared property contains a reference to its origin the method returns
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
