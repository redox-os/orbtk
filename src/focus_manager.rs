use std::cell::RefCell;
use std::sync::Arc;
use widgets::Widget;

pub struct FocusManager {
    focused_widget: RefCell<Option<Arc<Widget>>>
}

impl FocusManager {
    pub fn new() -> Self {
        FocusManager {
            focused_widget: RefCell::new(None),
        }
    }

    pub fn request_focus(&self, widget: &Arc<Widget>) {
        (*self.focused_widget.borrow_mut()) = Some(widget.clone());
    }

    pub fn focused(&self, widget: &Arc<Widget>) -> bool {
        if let Some(ref focused_widget) = *self.focused_widget.borrow_mut() {
            if Arc::ptr_eq(&widget, &focused_widget) {
                return true
            } 
        }

        false
    }
}