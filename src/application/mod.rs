//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, World};

use crate::prelude::*;
use crate::backend::*;
use crate::systems::*;

pub use self::global::*;
pub use self::window::*;

mod global;
mod window;

/// The `Application` represents the entry point of an OrbTk based application.
#[derive(Default)]
pub struct Application {
    windows: Vec<WindowShell>,
}

impl Application {
    /// Creates a new application.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn window<F: Fn(&mut BuildContext) -> Entity + 'static>(mut self, create_fn: F) -> Self {
        let mut world = World::from_container(Tree::default());

        let render_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let layouts = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));
        let update = Rc::new(Cell::new(true));
        let running = Rc::new(Cell::new(true));

        let mut context = BuildContext::new(
            &mut world,
            render_objects.clone(),
            layouts.clone(),
            handlers.clone(),
            states.clone(),
        );

        let window = create_fn(&mut context);

        {
            let tree: &mut Tree = world.entity_container();
            tree.root = window;
        }

        let title = world.entity_component_manager().borrow_component::<Title>(window).unwrap().clone();
        let resizeable = world.entity_component_manager().borrow_component::<Resizeable>(window).unwrap().clone();
        let position = world.entity_component_manager().borrow_component::<Pos>(window).unwrap().clone();
        let constraint = world.entity_component_manager().borrow_component::<Constraint>(window).unwrap().clone();

        world.entity_component_manager().register_component(window, Global::default());
        world.entity_component_manager().register_component(window, Global::default());
        world.entity_component_manager().register_component(window, Bounds::from((0.0, 0.0, constraint.width(), constraint.height())));

        let (mut runner, backend) =
            target_backend(&title.0, Bounds::from((position.0.x, position.0.y, constraint.width(), constraint.height())), false);

        world.register_init_system(InitSystem {
            backend: backend.clone(),
            states: states.clone(),
        });

        world
            .create_system(EventSystem {
                backend: backend.clone(),
                handlers: handlers.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(StateSystem {
                backend: backend.clone(),
                states: states.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(1)
            .build();

        world
            .create_system(LayoutSystem {
                backend: backend.clone(),
                layouts: layouts.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(2)
            .build();

        world
            .create_system(PostLayoutStateSystem {
                backend: backend.clone(),
                states: states.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(3)
            .build();

        world
            .create_system(RenderSystem {
                backend: backend.clone(),
                render_objects: render_objects.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(4)
            .build();

        runner.world(world);

        self.windows.push(WindowShell {
            backend_runner: runner,
            render_objects,
            layouts,
            handlers,
            states,
            update,
            running,
            resizable: resizeable.0,
        });

        self
    }

    /// Starts the application and run it until quit is requested.
    pub fn run(&mut self) {
        for window in &mut self.windows {
            window.run();
        }
    }
}
