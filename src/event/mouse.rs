use std::any::TypeId;
use std::rc::Rc;

use {EventHandler, EventBox, Entity, EntityComponentManager, Rect};

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub enum MouseEvent {
    Move((i32, i32)),
    Down(MouseButton, (i32, i32)),
    Up(MouseButton),
}

pub type OnMouseDown = Rc<Fn() + 'static>;

pub struct MouseDownHandler {
    func: Rc<Fn()>,
}

impl MouseDownHandler {
    pub fn new(func: Rc<Fn()>) -> Rc<Self> {
        Rc::new(MouseDownHandler { func })
    }
}

impl EventHandler for MouseDownHandler {
    fn emit(&self) -> bool {
        (self.func)();
        false
    }

    fn event_type(&self) -> TypeId {
        TypeId::of::<MouseEvent>()
    }

    fn check_condition(&self, event: &EventBox, entity: Entity, ecm: &mut EntityComponentManager) -> bool {
        if let Ok(mouse_event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent::Down(_button, position) = mouse_event {
                if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
                    if position.0 >= bounds.x && position.0 <= bounds.x + bounds.width as i32 && position.1 >= bounds.y && bounds.y <= bounds.y + bounds.height as i32 {
                        return true
                    }
                }
            }
        }
        
        false
    }
}
