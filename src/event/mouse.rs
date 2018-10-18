use std::rc::Rc;

use {Entity, EntityComponentManager, Event, Point, Rect};

pub fn check_mouse_condition(
    position: (i32, i32),
    entity: Entity,
    ecm: &mut EntityComponentManager,
) -> bool {
    if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
        let mut global_pos = (0, 0);

        if let Ok(g_pos) = ecm.borrow_component::<Point>(entity) {
            global_pos = (g_pos.x, g_pos.y);
        }

        if position.0 >= global_pos.0
            && position.0 <= global_pos.0 + bounds.width as i32
            && position.1 >= global_pos.1
            && position.1 <= global_pos.1 + bounds.height as i32
        {
            return true;
        }
    }

    false
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub struct MouseMouveEvent {
    pub position: (i32, i32),
}

impl Event for MouseMouveEvent {}

pub struct MouseUpEvent {
    pub button: MouseButton,
    pub position: (i32, i32),
}

impl Event for MouseUpEvent {}

pub struct MouseDownEvent {
    pub button: MouseButton,
    pub position: (i32, i32),
}

impl Event for MouseDownEvent {}

pub type MouseHandler = Rc<Fn() + 'static>;

pub type OnMouseUp = Rc<Fn() + 'static>;