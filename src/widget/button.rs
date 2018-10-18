use std::any::TypeId;
use std::rc::Rc;
use theme::Selector;
use {
    check_mouse_condition, Container, Entity, EntityComponentManager, EventBox, MouseDownEvent,
    MouseHandler, MouseUpEvent, Property, State, Template, TextBlock, Tree, Widget,
};

struct Pressed(bool);

#[derive(Default)]
pub struct ButtonState {
    pub on_mouse_down: Option<MouseHandler>,
    pub on_mouse_up: Option<MouseHandler>,
}

impl State for ButtonState {
    fn handles_event(
        &self,
        event: &EventBox,
        entity: Entity,
        ecm: &mut EntityComponentManager,
    ) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            return check_mouse_condition(event.position, entity, ecm);
        }

        if event.event_type() == TypeId::of::<MouseUpEvent>() {
            return ecm.borrow_component::<Pressed>(entity).unwrap().0;
        }

        false
    }

    fn update(
        &self,
        event: &EventBox,
        entity: Entity,
        tree: &Tree,
        ecm: &mut EntityComponentManager,
    ) -> bool {
        fn add_selector(
            pseudo_class: &str,
            entity: Entity,
            tree: &Tree,
            ecm: &mut EntityComponentManager,
        ) {
            if let Ok(selector) = ecm.borrow_mut_component::<Selector>(entity) {
                selector.pseudo_classes.insert(String::from(pseudo_class));
            }

            for child in &tree.children[&entity] {
                add_selector(pseudo_class, *child, tree, ecm);
            }
        }

        fn remove_selector(
            pseudo_class: &str,
            entity: Entity,
            tree: &Tree,
            ecm: &mut EntityComponentManager,
        ) {
            if let Ok(selector) = ecm.borrow_mut_component::<Selector>(entity) {
                selector.pseudo_classes.remove(&String::from(pseudo_class));
            }

            for child in &tree.children[&entity] {
                remove_selector(pseudo_class, *child, tree, ecm);
            }
        }

        if event.event_type() == TypeId::of::<MouseDownEvent>() {
            add_selector("active", entity, tree, ecm);
            ecm.borrow_mut_component::<Pressed>(entity).unwrap().0 = true;
            if let Some(handler) = &self.on_mouse_down {
                (handler)();
            }

            return true;
        }

        if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
            remove_selector("active", entity, tree, ecm);
            ecm.borrow_mut_component::<Pressed>(entity).unwrap().0 = false;

            if check_mouse_condition(event.position, entity, ecm) {
                if let Some(handler) = &self.on_mouse_up {
                    (handler)();
                }
            }

            return true;
        }

        false
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property::new(Pressed(false))]
    }
}

pub struct Button {
    pub label: String,
    pub class: String,
    pub state: Rc<State>,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            label: String::from("Button"),
            class: String::from("button"),
            state: Rc::new(ButtonState {
                ..Default::default()
            }),
        }
    }
}

impl Widget for Button {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Container {
            class: self.class.clone(),
            child: Some(Rc::new(TextBlock {
                label: self.label.clone(),
                class: self.class.clone(),
                key: String::from("Label"),
            })),
        }))
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property::new(Selector::new(Some(self.class.clone())))]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }
}
