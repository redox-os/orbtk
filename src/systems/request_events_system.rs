use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use dces::{Entity, EntityComponentManager, System};

use application::{Global, Tree};
use event::SystemEvent;

pub struct RequestEventsSystem {
    pub running: Arc<atomic::AtomicBool>,
}

impl System<Tree> for RequestEventsSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if let Ok(global) = ecm.borrow_mut_component::<Global>(0) {
            if let Some(window) = &mut global.window {
                for event in window.events() {
                    match event {
                        orbrender::events::Event::System(system_event) => match system_event {
                            orbrender::events::SystemEvent::Quit => {
                                self.running.store(false, atomic::Ordering::Release);
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
               
            }
        }
    }
}
