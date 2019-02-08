use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, World};

use crate::{
    application::{Application, Tree},
    backend::{target_backend, BackendRunner},
    event::EventHandler,
    layout::Layout,
    properties::{Bounds, Constraint},
    render_object::RenderObject,
    structs::Size,
    shapes::UpdateableShape,
    systems::{
        EventSystem, InitSystem, LayoutSystem, PostLayoutStateSystem, RenderSystem, StateSystem,
    },
    theme::Theme,
    widget::{PropertyResult, State, Template},
    Global,
};

/// Represents a window. Each window has its own tree, event pipeline and backend.
pub struct Window {
    pub backend_runner: Box<dyn BackendRunner>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub shapes: Rc<RefCell<BTreeMap<Entity, Box<dyn UpdateableShape>>>>,
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
    pub root: Option<Template>,
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

    /// Used to set the `root` template of the window.
    pub fn root(mut self, root: Template) -> Self {
        self.root = Some(root);
        self
    }

    /// Sets whether the window is resizable or not.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Used to set the `debug` flag of the window.
    /// If the flag is set to `ture` debug informations will be printed to the console.
    pub fn debug_flag(mut self, debug: bool) -> Self {
        self.debug_flag = debug;
        self
    }

    /// Creates the window with the given properties and builds its widget tree.
    pub fn build(self) {
        let (mut runner, backend) =
            target_backend(&self.title, self.bounds, self.resizable, self.theme);
        let mut world = World::from_container(Tree::default());
        let render_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let shapes = Rc::new(RefCell::new(BTreeMap::new()));
        let layouts = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));
        let update = Rc::new(Cell::new(true));
        let running = Rc::new(Cell::new(true));
        let debug_flag = Rc::new(Cell::new(self.debug_flag));

        if debug_flag.get() {
            println!("------ Start build tree ------\n");
        }

        // register window as entity with global properties
        if world.entity_container().is_empty() {
            let window = world
                .create_entity()
                .with(Global::default())
                .with(Bounds::new(
                    0.0,
                    0.0,
                    self.bounds.width(),
                    self.bounds.height(),
                ))
                .with(Constraint::default())
                .build();

            if debug_flag.get() {
                println!("Window (id = {}, children_len = 1)", window,);
            }
        }

        if let Some(root) = self.root {
            build_tree(
                root,
                &mut world,
                &render_objects,
                &shapes,
                &layouts,
                &handlers,
                &states,
                &debug_flag,
            );
        }

        world.register_init_system(InitSystem {
            backend: backend.clone(),
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
                shapes: shapes.clone(),
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
                shapes: shapes.clone(),
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
            shapes,
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

// Builds the widget tree.
fn build_tree(
    root: Template,
    world: &mut World<Tree>,
    render_objects: &Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    shapes: &Rc<RefCell<BTreeMap<Entity, Box<dyn UpdateableShape>>>>,
    layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    handlers: &Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    states: &Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    debug_flag: &Rc<Cell<bool>>,
) {
    fn expand(
        world: &mut World<Tree>,
        render_objects: &Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
        shapes: &Rc<RefCell<BTreeMap<Entity, Box<dyn UpdateableShape>>>>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: &Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
        states: &Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
        template: Template,
        debug_flag: &Rc<Cell<bool>>,
        depth: usize,
    ) -> Entity {
        let mut template = template;

        let entity = {
            let mut entity_builder = world.create_entity();

            // normal properties
            for (_, value) in template.properties.drain() {
                entity_builder = entity_builder.with_box(value);
            }

            // shared property
            for (_, value) in template.shared_properties {
                match value.build() {
                    PropertyResult::Property(property, source) => {
                        entity_builder = entity_builder.with_box(property);
                        source.set(Some(entity_builder.entity));
                    }
                    PropertyResult::Source(source) => {
                        entity_builder = entity_builder.with_shared_box(source);
                    }
                    PropertyResult::PropertyNotFound => {}
                }
            }

            // constraint
            entity_builder = entity_builder.with(template.constraint);

            let entity = entity_builder.build();

            if let Some(render_object) = template.render_object {
                render_objects.borrow_mut().insert(entity, render_object);
            }

            layouts.borrow_mut().insert(entity, template.layout);

            if let Some(shape) = template.shape {
                shapes.borrow_mut().insert(entity, shape);
            }

            let widget_handlers = template.event_handlers;

            if !widget_handlers.is_empty() {
                let mut event_handlers = vec![];

                for handler in widget_handlers {
                    event_handlers.push(handler.clone());
                }

                handlers.borrow_mut().insert(entity, event_handlers);
            }

            if let Some(state) = template.state {
                states.borrow_mut().insert(entity, state.clone());
            }

            entity
        };

        if debug_flag.get() {
            println!(
                "{}{} (id = {}, children_len = {})",
                "| ".repeat(depth),
                template.debug_name,
                entity,
                template.children.len()
            );
        }

        if world.entity_container().len() == 2 {
            let root = world.entity_container().root;
            let _result = world.entity_container().append_child(root, entity);
        }

        for child in template.children.drain(0..) {
            let child = expand(
                world,
                render_objects,
                shapes,
                layouts,
                handlers,
                states,
                child,
                debug_flag,
                depth + 1,
            );
            let _result = world.entity_container().append_child(entity, child);
        }

        entity
    }

    expand(
        world,
        render_objects,
        shapes,
        layouts,
        handlers,
        states,
        root,
        debug_flag,
        1,
    );

    if debug_flag.get() {
        println!("\n------  End build tree  ------ ");
    }
}
