use std::rc::Rc;

use {Entity, EntityComponentManager, Event, Point, Rect};

pub fn check_mouse_condition(
    position: Point,
    entity: Entity,
    ecm: &mut EntityComponentManager,
) -> bool {
    if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
        let mut rect = Rect::new(0, 0, bounds.width, bounds.height);

        if let Ok(g_pos) = ecm.borrow_component::<Point>(entity) {
            rect.x = g_pos.x;
            rect.y = g_pos.y;
        }

        return rect.contains(position)
    }

    false  
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub struct MouseMouveEvent {
    pub position: Point,
}

impl Event for MouseMouveEvent {}

pub struct MouseUpEvent {
    pub button: MouseButton,
    pub position: Point,
}

impl Event for MouseUpEvent {}

pub struct MouseDownEvent {
    pub button: MouseButton,
    pub position: Point,
}

impl Event for MouseDownEvent {}

pub type MouseHandler = Rc<Fn() + 'static>;

pub type OnMouseUp = Rc<Fn() + 'static>;