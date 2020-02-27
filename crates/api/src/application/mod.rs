//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, World};

use crate::{
    prelude::*,
    shell::{ShellRunner, WindowBuilder},
    tree::*,
    utils::{Point, Rectangle},
};

pub use self::global::*;
pub use self::overlay::*;
pub use self::window::*;

mod global;
mod overlay;
mod window;

/// The `Application` represents the entry point of an OrbTk based application.
#[derive(Default)]
pub struct Application {
    runners: Vec<ShellRunner<WindowAdapter>>,
    name: Box<str>,
}

impl Application {
    /// Creates a new application.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new application with the given name.
    pub fn from_name(name: impl Into<Box<str>>) -> Self {
        Application {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Creates a new window and add it to the application.
    pub fn window<F: Fn(&mut BuildContext) -> Entity + 'static>(mut self, create_fn: F) -> Self {
        let mut world = World::from_stores(Tree::default(), StringComponentStore::default());

        let render_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let layouts = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));
        let registry = Rc::new(RefCell::new(Registry::new()));

        // register settings service.
        if self.name.is_empty() {
            registry
                .borrow_mut()
                .register("settings", Settings::default());
        } else {
            registry
                .borrow_mut()
                .register("settings", Settings::new(&*self.name));
        };

        let window = {
            let overlay = Overlay::create().build(&mut BuildContext::new(
                world.entity_component_manager(),
                &render_objects,
                &mut layouts.borrow_mut(),
                &mut handlers.borrow_mut(),
                &mut states.borrow_mut(),
                &mut crate::theme::default_theme(),
            ));

            {
                let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
                tree.set_overlay(overlay);
            }

            let window = create_fn(&mut BuildContext::new(
                world.entity_component_manager(),
                &render_objects,
                &mut layouts.borrow_mut(),
                &mut handlers.borrow_mut(),
                &mut states.borrow_mut(),
                &mut crate::theme::default_theme(),
            ));

            {
                let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
                tree.set_root(window);
            }

            window
        };

        let title = world
            .entity_component_manager()
            .component_store()
            .get::<String>("title", window)
            .unwrap()
            .clone();
        let borderless = *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("borderless", window)
            .unwrap();
        let resizeable = *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("resizeable", window)
            .unwrap();
        let position = *world
            .entity_component_manager()
            .component_store()
            .get::<Point>("position", window)
            .unwrap();
        let constraint = *world
            .entity_component_manager()
            .component_store()
            .get::<Constraint>("constraint", window)
            .unwrap();

        world
            .entity_component_manager()
            .component_store_mut()
            .register("global", window, Global::default());
        world
            .entity_component_manager()
            .component_store_mut()
            .register("global", window, Global::default());
        world
            .entity_component_manager()
            .component_store_mut()
            .register(
                "bounds",
                window,
                Rectangle::from((0.0, 0.0, constraint.width(), constraint.height())),
            );

        let window_shell = Rc::new(RefCell::new(
            WindowBuilder::new(WindowAdapter {
                root: window,
                render_objects: render_objects.clone(),
                layouts: layouts.clone(),
                handlers: handlers.clone(),
                states: states.clone(),
                ..Default::default()
            })
            .title(&(title)[..])
            .bounds(Rectangle::from((
                position.x,
                position.y,
                constraint.width(),
                constraint.height(),
            )))
            .borderless(borderless)
            .resizeable(resizeable)
            .build(),
        ));

        #[cfg(not(target_arch = "wasm32"))]
        window_shell
            .borrow_mut()
            .render_context_2_d()
            .register_font("Roboto Regular", crate::theme::fonts::ROBOTO_REGULAR_FONT);

        #[cfg(not(target_arch = "wasm32"))]
        window_shell
            .borrow_mut()
            .render_context_2_d()
            .register_font("Roboto Medium", crate::theme::fonts::ROBOTO_MEDIUM_FONT);

        #[cfg(not(target_arch = "wasm32"))]
        window_shell
            .borrow_mut()
            .render_context_2_d()
            .register_font(
                "Material Icons",
                crate::theme::fonts::MATERIAL_ICONS_REGULAR_FONT,
            );

        world.register_init_system(InitSystem {
            shell: window_shell.clone(),
            layouts: layouts.clone(),
            render_objects: render_objects.clone(),
            handlers: handlers.clone(),
            states: states.clone(),
            registry: registry.clone(),
        });

        world.register_cleanup_system(CleanupSystem {
            shell: window_shell.clone(),
            layouts: layouts.clone(),
            render_objects: render_objects.clone(),
            handlers: handlers.clone(),
            states: states.clone(),
            registry: registry.clone(),
        });

        world
            .create_system(EventStateSystem {
                shell: window_shell.clone(),
                handlers: handlers.clone(),
                mouse_down_nodes: RefCell::new(vec![]),
                render_objects: render_objects.clone(),
                states: states.clone(),
                layouts: layouts.clone(),
                registry: registry.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(LayoutSystem {
                shell: window_shell.clone(),
                layouts: layouts.clone(),
            })
            .with_priority(1)
            .build();

        world
            .create_system(PostLayoutStateSystem {
                shell: window_shell.clone(),
                layouts: layouts.clone(),
                render_objects: render_objects.clone(),
                handlers: handlers.clone(),
                states: states.clone(),
                registry: registry.clone(),
            })
            .with_priority(2)
            .build();

        world
            .create_system(RenderSystem {
                shell: window_shell.clone(),
                layouts: layouts.clone(),
                render_objects: render_objects.clone(),
                handlers: handlers.clone(),
                states: states.clone(),
            })
            .with_priority(3)
            .build();

        self.runners.push(ShellRunner {
            updater: Box::new(WorldWrapper { world }),
            window_shell,
        });

        self
    }

    /// Starts the application and run it until quit is requested.
    pub fn run(mut self) {
        while let Some(runner) = self.runners.pop() {
            #[cfg(not(target_arch = "wasm32"))]
            let mut runner = runner;

            #[cfg(target_arch = "wasm32")]
            let runner = runner;
            runner.run();
        }
    }
}
