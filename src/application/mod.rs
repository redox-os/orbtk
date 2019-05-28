//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, World};

use crate::prelude::*;
use crate::shell::{ShellRunner, WindowBuilder};
use crate::systems::*;

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

        let window_shell = Rc::new(RefCell::new(WindowBuilder::new(WindowAdapter {
            root: window,
            render_objects: render_objects.clone(),
            layouts: layouts.clone(),
            handlers: handlers.clone(),
            states: states.clone(),
            ..Default::default()
        }).title(&(title.0)[..]).bounds(Bounds::from((position.0.x, position.0.y, constraint.width(), constraint.height()))).resizeable(resizeable.0).build()));

        world.register_init_system(InitSystem {
            shell: window_shell.clone(),
            states: states.clone(),
        });

        world
            .create_system(EventSystem {
                shell: window_shell.clone(),
                handlers: handlers.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(StateSystem {
                shell: window_shell.clone(),
                states: states.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(1)
            .build();

        world
            .create_system(LayoutSystem {
                shell: window_shell.clone(),
                layouts: layouts.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(2)
            .build();

        world
            .create_system(PostLayoutStateSystem {
                shell: window_shell.clone(),
                states: states.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(3)
            .build();

        world
            .create_system(RenderSystem {
                shell: window_shell.clone(),
                render_objects: render_objects.clone(),
                update: update.clone(),
                running: running.clone(),
            })
            .with_priority(4)
            .build();

        self.runners.push(ShellRunner {
            updater: Box::new(WorldWrapper {
                world
            }),
            window_shell,
            update,
            running,
        });

        self
    }

    /// Starts the application and run it until quit is requested.
    pub fn run(mut self) {
        while let Some(runner) = self.runners.pop() {
            let mut runner = runner;
            runner.run();
        }
    }
}
