use std::any::TypeId;
use std::rc::Rc;

use {Entity, EntityComponentManager, EventBox, EventHandler, Point, Rect, State, Tree};

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub enum MouseEvent {
    Move((i32, i32)),
    Down(MouseButton, (i32, i32)),
    Up(MouseButton, (i32, i32)),
}

pub struct MouseUpEvent {
    pub button: MouseButton,
    pub position: (i32, i32),
}

pub struct MouseDownEvent {
    pub button: MouseButton,
    pub position: (i32, i32),
}

pub type OnMouseDown = Rc<Fn() + 'static>;

pub struct MouseDownHandler {
    func: Rc<Fn()>,
    state: Box<State>,
}

impl MouseDownHandler {
    pub fn new(state: Box<State>, func: Rc<Fn()>) -> Rc<Self> {
        Rc::new(MouseDownHandler { func, state })
    }
}

impl EventHandler for MouseDownHandler {
    fn emit(&self) -> bool {
        (self.func)();
        false
    }

    fn update(&self, entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager) {
        self.state.update(entity, tree, ecm);
    }

    fn event_type(&self) -> TypeId {
        TypeId::of::<MouseDownEvent>()
    }

    fn check_condition(
        &self,
        event: &EventBox,
        entity: Entity,
        ecm: &mut EntityComponentManager,
    ) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
                let mut global_pos = (0, 0);

                let position = event.position;

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
        }

        false
    }
}

pub type OnMouseUp = Rc<Fn() + 'static>;

pub struct MouseUpHandler {
    func: Rc<Fn()>,
    state: Box<State>,
}

impl MouseUpHandler {
    pub fn new(state: Box<State>, func: Rc<Fn()>) -> Rc<Self> {
        Rc::new(MouseUpHandler { func, state })
    }
}

impl EventHandler for MouseUpHandler {
    fn emit(&self) -> bool {
        (self.func)();
        false
    }

    fn update(&self, entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager) {
        self.state.update(entity, tree, ecm);
    }

    fn event_type(&self) -> TypeId {
        TypeId::of::<MouseUpEvent>()
    }

    fn check_condition(
        &self,
        event: &EventBox,
        entity: Entity,
        ecm: &mut EntityComponentManager,
    ) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
            if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {

                // todo check if pressed!!!
                let mut global_pos = (0, 0);

                let position = event.position;

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
        }

        false
    }
}
