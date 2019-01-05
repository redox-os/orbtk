use std::sync::Arc;
use std::sync::atomic::{self, AtomicBool};

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use dces::{Entity, EntityComponentManager, System};

use application::{Tree, Global};

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
pub struct RenderSystem {
    pub update: Arc<atomic::AtomicBool>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.update.load(atomic::Ordering::Acquire) || tree.is_empty() {
            return;
        }

        if let Ok(global) = ecm.borrow_mut_component::<Global>(0) {
            if let Some(window) = &mut global.window {
                window.render();
                self.update.store(false, atomic::Ordering::Release);
            }
        }
    }
}
