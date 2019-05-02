use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, World};

use crate::prelude::*;
use crate::backend::*;
use crate::systems::*;

/// Represents a window. Each window has its own tree, event pipeline and backend.
pub struct Window {
    pub backend_runner: Box<dyn BackendRunner>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub resizable: bool,
    pub debug_flag: Rc<Cell<bool>>,
}

impl Window {
    /// Executes the given window until quit is requested.
    pub fn run(&mut self) {
        self.backend_runner
            .run(self.update.clone(), self.running.clone());
    }
}

/// The `WindowBuilder` is used to define and build a `Window`.
pub struct WindowBuilder<'a> {
    pub application: &'a mut Application,
    pub bounds: Bounds,
    pub title: String,
    pub theme: Theme,
    // pub root: Option<Template>,
    pub resizable: bool,
    pub debug_flag: bool,
}

impl<'a> WindowBuilder<'a> {
    /// Used to define the render `bounds` of the window.
    pub fn bounds<B: Into<Bounds>>(mut self, bounds: B) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Used to set the `title` of the window.
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    /// Used to set the css `theme` of the window.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets whether the window is resizable or not.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Used to set the `debug` flag of the window.
    /// If the flag is set to `true` debug information will be printed to the console.
    pub fn debug_flag(mut self, debug: bool) -> Self {
        self.debug_flag = debug;
        self
    }

    /// Creates the window with the given properties and builds its widget tree.
    pub fn build<W>(self, root: W)
        where
            W: Widget,
    {
        let mut world = World::from_container(Tree::default());

        // register window as entity with global properties
        let window = world
            .create_entity()
            .with(Global::default())
            .with(Name::from("Window"))
            .with(Point::default())
            .with(Bounds::from(Rect::new(
                0.0,
                0.0,
                self.bounds.width(),
                self.bounds.height(),
            )))
            .with(Constraint::default())
            .build();

        let (mut runner, backend) =
            target_backend(&self.title, self.bounds, self.resizable, self.theme);

        let render_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let layouts = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));
        let update = Rc::new(Cell::new(true));
        let running = Rc::new(Cell::new(true));
        let debug_flag = Rc::new(Cell::new(self.debug_flag));

        let mut context = BuildContext::new(
            &mut world,
            render_objects.clone(),
            layouts.clone(),
            handlers.clone(),
            states.clone(),
        );

        // Register root widget as child of window
        let root = root.build(&mut context);
        world.entity_container().append_child(window, root).unwrap();

        world.register_init_system(InitSystem {
            backend: backend.clone(),
            states: states.clone(),
            debug_flag: debug_flag.clone(),
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
                debug_flag: debug_flag.clone(),
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
                debug_flag: debug_flag.clone(),
                running: running.clone(),
            })
            .with_priority(4)
            .build();

        runner.world(world);

        self.application.windows.push(Window {
            backend_runner: runner,
            render_objects,
            layouts,
            handlers,
            states,
            update,
            running,
            resizable: self.resizable,
            debug_flag,
        })
    }
}
