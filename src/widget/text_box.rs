use std::any::TypeId;
use std::rc::Rc;

use dces::{Entity, EntityComponentManager};

use super::Property;
use event::{check_mouse_condition, EventBox, KeyDownEvent, Key, MouseDownEvent};
use state::State;
use theme::Selector;
use tree::Tree;
use widget::{Container, Label, Template, TextBlock, Widget};

// todo: cursor struct with position and selection length

pub struct Focused(pub bool);

#[derive(Default)]
pub struct TextCursor {
    pub position: u32,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[derive(Default)]
pub struct TextBoxState {}

impl State for TextBoxState {
    fn handles_event(
        &self,
        event: &EventBox,
        entity: Entity,
        ecm: &mut EntityComponentManager,
    ) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            let mut focused = false;
            if let Ok(foc) = ecm.borrow_component::<Focused>(entity) {
                focused = foc.0;
            }

            return !focused && check_mouse_condition(event.position, entity, ecm);
        }

        if event.event_type() == TypeId::of::<KeyDownEvent>() {
            if let Ok(foc) = ecm.borrow_component::<Focused>(entity) {
                return foc.0;
            }
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

        fn update_label(key: &Key, entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager) {
            if let Ok(label) = ecm.borrow_mut_component::<Label>(entity) {
                if key.to_string() != "" {
                    label.0.push_str(&key.to_string());
                } else {
                    match *key {
                        Key::Backspace => {
                            label.0.pop();
                        },
                        _ => {}
                    }
                }
            }

            for child in &tree.children[&entity] {
                update_label(key, *child, tree, ecm);
            }
        }

        // fn remove_selector(
        //     pseudo_class: &str,
        //     entity: Entity,
        //     tree: &Tree,
        //     ecm: &mut EntityComponentManager,
        // ) {
        //     if let Ok(selector) = ecm.borrow_mut_component::<Selector>(entity) {
        //         selector.pseudo_classes.remove(&String::from(pseudo_class));
        //     }

        //     for child in &tree.children[&entity] {
        //         remove_selector(pseudo_class, *child, tree, ecm);
        //     }
        // }

        if event.event_type() == TypeId::of::<MouseDownEvent>() {
            add_selector("active", entity, tree, ecm);
            ecm.borrow_mut_component::<Focused>(entity).unwrap().0 = true;

            return true;
        }

        if let Ok(key_event) = event.downcast_ref::<KeyDownEvent>() {
            update_label(&key_event.key, entity, tree, ecm);
            return true;
        }

        false
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            Property::new(Focused(false)),
            Property::new(TextCursor::default()),
        ]
    }
}

pub struct TextBox {
    pub label: String,
    pub class: String,
    pub state: Rc<State>,
}

impl Default for TextBox {
    fn default() -> TextBox {
        TextBox {
            label: String::from("TextBox"),
            class: String::from("textbox"),
            state: Rc::new(TextBoxState {
                ..Default::default()
            }),
        }
    }
}

impl Widget for TextBox {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Container {
            class: self.class.clone(),
            child: Some(Rc::new(TextBlock {
                label: self.label.clone(),
                class: self.class.clone(),
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
