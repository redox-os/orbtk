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
    Up(MouseButton),
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
        TypeId::of::<MouseEvent>()
    }

    fn check_condition(
        &self,
        event: &EventBox,
        entity: Entity,
        ecm: &mut EntityComponentManager,
    ) -> bool {
        if let Ok(mouse_event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent::Down(_button, position) = mouse_event {
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
            }
        }

        false
    }
}
