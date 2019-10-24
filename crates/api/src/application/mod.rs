//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, World};

use crate::{
    prelude::*,
    shell::{ShellRunner, WindowBuilder},
    tree::*,
};

pub use self::global::*;
pub use self::window::*;

mod global;
mod window;

/// The `Application` represents the entry point of an OrbTk based application.
#[derive(Default)]
pub struct Application {
    runners: Vec<ShellRunner<WindowAdapter>>,
}

impl Application {
    /// Creates a new application.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn window<F: Fn(&mut BuildContext) -> Entity + 'static>(mut self, create_fn: F) -> Self {
        let mut world = World::from_stores(Tree::default(), StringComponentStore::default());

        let render_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let layouts = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));
        let update = Rc::new(Cell::new(true));
        let running = Rc::new(Cell::new(true));

        let window = {
            let mut context = BuildContext::new(
                world.entity_component_manager(),
                render_objects.clone(),
                layouts.clone(),
                handlers.clone(),
                states.clone(),
            );

            create_fn(&mut context)
        };

        {
            let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
            tree.set_root(window);
        }

        let title = world
            .entity_component_manager()
            .component_store()
            .borrow_component::<String>("title", window)
            .unwrap()
            .clone();
        let resizeable = world
            .entity_component_manager()
            .component_store()
            .borrow_component::<bool>("resizeable", window)
            .unwrap()
            .clone();
        let position = world
            .entity_component_manager()
            .component_store()
            .borrow_component::<Pos>("position", window)
            .unwrap()
            .clone();
        let constraint = world
            .entity_component_manager()
            .component_store()
            .borrow_component::<Constraint>("constraint", window)
            .unwrap()
            .clone();

        world
            .entity_component_manager()
            .component_store_mut()
            .register_component("global", window, Global::default());
        world
            .entity_component_manager()
            .component_store_mut()
            .register_component("global", window, Global::default());
        world
            .entity_component_manager()
            .component_store_mut()
            .register_component(
                "bounds",
                window,
                Bounds::from((0.0, 0.0, constraint.width(), constraint.height())),
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
            .bounds(Bounds::from((
                position.0.x,
                position.0.y,
                constraint.width(),
                constraint.height(),
            )))
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
        });

        world
            .create_system(EventStateSystem {
                shell: window_shell.clone(),
                handlers: handlers.clone(),
                update: update.clone(),
                running: running.clone(),
                mouse_down_nodes: RefCell::new(vec![]),
                render_objects: render_objects.clone(),
                states: states.clone(),
                layouts: layouts.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(LayoutSystem {
                shell: window_shell.clone(),
                layouts: layouts.clone(),
                update: update.clone(),
                running: running.clone(),
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
                update: update.clone(),
                running: running.clone(),
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
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(3)
            .build();

        self.runners.push(ShellRunner {
            updater: Box::new(WorldWrapper { world }),
            window_shell,
            update,
            running,
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
