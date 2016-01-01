pub use std::cell::Cell as CopyCell;

use std::cell::{Ref, RefCell, RefMut};

pub struct CloneCell<T: Clone> {
    inner: RefCell<T>,
}

impl<T: Clone> CloneCell<T> {
    pub fn new(value: T) -> Self {
        CloneCell { inner: RefCell::new(value) }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

    pub fn get(&self) -> T {
        self.inner.borrow().clone()
    }

    pub fn set(&self, value: T) {
        *self.inner.borrow_mut() = value;
    }
}
