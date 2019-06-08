use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::collections::BTreeMap;

use dces::{Entity, World};

use application::{Application, Tree};
use backend::{target_backend, BackendRunner};
use event::EventHandler;
use layout_object::{RootLayoutObject, LayoutObject};
use render_object::RenderObject;
use properties::{Point, Rect};
use systems::{EventSystem, LayoutSystem, RenderSystem, StateSystem};
use theme::Theme;
use widget::{PropertyResult, State, Template};
use Global;

/// Represents a window. Each window has its own tree, event pipline and backend.
pub struct Window {
    pub backend_runner: Box<BackendRunner>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
    pub layout_objects: Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<EventHandler>>>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
    pub update: Rc<Cell<bool>>,
    pub debug_flag: Rc<Cell<bool>>,
}

impl Window {
    /// Executes the given window unitl quit is requested.
    pub fn run(&mut self) {
        self.backend_runner.run(self.update.clone());
    }
}

/// The `WindowBuilder` is used to define and build a `Window`.
pub struct WindowBuilder<'a> {
    pub application: &'a mut Application,
    pub bounds: Rect,
    pub title: String,
    pub theme: Theme,
    pub root: Option<Template>,
    pub debug_flag: bool,
}

impl<'a> WindowBuilder<'a> {
    /// Used to define the render `bounds` of the window.
    pub fn with_bounds(mut self, bounds: Rect) -> Self {
        self.bounds = bounds;
        self
    }

    /// Used to set the `title` of the window.
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    /// Used to set the css `theme` of the window.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Used to set the `root` template of the window.
    pub fn with_root(mut self, root: Template) -> Self {
        self.root = Some(root);
        self
    }

    /// Used to set the `debug` flag of the window.
    /// If the flag is set to `ture` debug informations will be printed to the console.
    pub fn with_debug_flag(mut self, debug: bool) -> Self {
        self.debug_flag = debug;
        self
    }

    /// Creates the window with the given properties and builds its widget tree.
    pub fn build(self) {
        let (mut runner, backend) = target_backend(&self.title, self.bounds, self.theme);
        let mut world = World::from_container(Tree::default());
        let render_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let layout_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));
        let update = Rc::new(Cell::new(true));
        let debug_flag = Rc::new(Cell::new(self.debug_flag));

        if debug_flag.get() {
            println!("------ Start build tree ------\n");
        }

        if let Some(root) = self.root {
            build_tree(
                root,
                &mut world,
                &render_objects,
                &layout_objects,
                &handlers,
                &states,
                &debug_flag,
            );
        }

        world
            .create_system(EventSystem {
                backend: backend.clone(),
                handlers: handlers.clone(),
                update: update.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(StateSystem {
                states: states.clone(),
                update: update.clone(),
            })
            .with_priority(1)
            .build();

        world
            .create_system(LayoutSystem {
                backend: backend.clone(),
                layout_objects: layout_objects.clone(),
                update: update.clone(),
            })
            .with_priority(2)
            .build();

        world
            .create_system(RenderSystem {
                backend: backend.clone(),
                render_objects: render_objects.clone(),
                update: update.clone(),
                debug_flag: debug_flag.clone(),
            })
            .with_priority(3)
            .build();

        runner.world(world);

        self.application.windows.push(Window {
            backend_runner: runner,
            render_objects,
            layout_objects,
            handlers,
            states,
            update,
            debug_flag,
        })
    }
}

// Builds the widget tree.
fn build_tree(
    root: Template,
    world: &mut World<Tree>,
    render_objects: &Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
    layout_objects: &Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,
    handlers: &Rc<RefCell<BTreeMap<Entity, Vec<Rc<EventHandler>>>>>,
    states: &Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
    debug_flag: &Rc<Cell<bool>>,
) {
    fn expand(
        world: &mut World<Tree>,
        render_objects: &Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
        layout_objects: &Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,
        handlers: &Rc<RefCell<BTreeMap<Entity, Vec<Rc<EventHandler>>>>>,
        states: &Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
        template: Template,
        debug_flag: &Rc<Cell<bool>>,
    ) -> Entity {
        // register window as entity with global properties
        if world.entity_container().is_empty() {
            let root = world
                .create_entity()
                .with(Global::default())
                .with(Rect::default())
                .with(Point::default())
                .build();

            layout_objects
                .borrow_mut()
                .insert(root, Box::new(RootLayoutObject));
        }

        let mut template = template;

        let entity = {
            let mut entity_builder = world
                .create_entity()
                .with(Rect::default())
                .with(Point::default());

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

            let entity = entity_builder.build();

            if let Some(render_object) = template.render_object {
                render_objects.borrow_mut().insert(entity, render_object);
            }

            layout_objects
                .borrow_mut()
                .insert(entity, template.layout_object);

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
                "{} (id = {}, children_length = {})",
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
                layout_objects,
                handlers,
                states,
                child,
                debug_flag,
            );
            let _result = world.entity_container().append_child(entity, child);
        }

        entity
    }

    expand(
        world,
        render_objects,
        layout_objects,
        handlers,
        states,
        root,
        debug_flag,
    );

    if debug_flag.get() {
        println!("\n------  End build tree  ------ ");
    }
}
