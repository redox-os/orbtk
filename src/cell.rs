use std::cell::{Cell, Ref, RefCell, RefMut};
use std::sync::Arc;

pub trait CheckSet<T> {
    fn check_set(&self, value: T) -> bool;
}

pub struct CloneCell<T: Clone + 'static> {
    inner: Arc<RefCell<T>>,
    change_callbacks: Arc<RefCell<Vec<Arc<dyn Fn(T)>>>>,
}

impl<T: Clone> CloneCell<T> {
    pub fn new(value: T) -> Self {
        CloneCell {
            inner: Arc::new(RefCell::new(value)),
            change_callbacks: Arc::new(RefCell::new(vec![])),
        }
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
        self.raise_changed((*self.inner.borrow()).clone());
    }

    pub fn bind(&self, other: &CloneCell<T>) {
        self.set(other.get());
        let self_value = self.inner.clone();
        let change_callbacks = self.change_callbacks.clone();

        other.on_changed(move |value: T| {
            *self_value.borrow_mut() = value.clone();

            for callback in &*change_callbacks.borrow() {
                callback(value.clone())
            }
        });
    }

    fn raise_changed(&self, value: T) {
        let change_callbacks = self.change_callbacks.clone();
        for callback in &*change_callbacks.borrow() {
            callback(value.clone())
        }
    }

    pub fn on_changed<F: Fn(T) + 'static>(&self, func: F) -> &Self {
        self.change_callbacks.borrow_mut().push(Arc::new(func));
        self
    }
}

impl<T: Copy> CheckSet<T> for Cell<T>
where
    T: PartialOrd,
{
    fn check_set(&self, value: T) -> bool {
        if value != self.get() {
            self.set(value);
            true
        } else {
            false
        }
    }
}

impl<T: Copy> CheckSet<T> for CloneCell<T>
where
    T: PartialOrd,
{
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
