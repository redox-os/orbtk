use std::cell::{Cell, Ref, RefCell, RefMut};

pub trait CheckSet<T> {
    fn check_set(&self, value: T) -> bool;
}

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

impl<T: Copy> CheckSet<T> for Cell<T> where T: PartialOrd {
    fn check_set(&self, value: T) -> bool {
        if value != self.get() {
            self.set(value);
            true
        } else {
            false
        }
    }
}

impl<T: Copy> CheckSet<T> for CloneCell<T> where T: PartialOrd {
    fn check_set(&self, value: T) -> bool {
        let mut borrow = self.inner.borrow_mut();
        if value != *borrow {
            *borrow = value;
            true
        } else {
            false
        }
    }
}
