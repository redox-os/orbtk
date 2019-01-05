use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

use std::collections::BTreeMap;

use dces::{Entity, World};
use orbrender;
use orbrender::backend::Runner;
use orbrender::render_objects::Rectangle;
use orbrender::traits;

use application::{Application, Tree};
// use backend::{target_backend, BackendRunner};
use event::EventHandler;
use layout_object::{LayoutObject, RootLayoutObject};
use properties::{Bounds, Point};
// use render_object::RenderObject;
use systems::{
    EventSystem, LayoutSystem, PostLayoutStateSystem, RenderSystem, RequestEventsSystem,
    StateSystem,
};
use theme::Theme;
use widget::{PropertyResult, State, Template};
use Global;

pub struct WindowSupplier<'a> {
    pub application: &'a mut Application,
    pub window: Box<traits::Window>,
    pub theme: Theme,
    pub root: Option<Template>,
    pub debug_flag: bool,
}

impl<'a> WindowSupplier<'a> {
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

    pub fn finish(self) {
        let mut window = self.window;
        let mut world = World::from_container(Tree::default());

        let layout_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));
        let update = Rc::new(Cell::new(true));
        let debug_flag = Rc::new(Cell::new(self.debug_flag));

        if self.debug_flag {
            println!("------ Start build tree ------\n");
        }

        if let Some(root) = self.root {
            build_tree(
                &mut window,
                root,
                &mut world,
                &layout_objects,
                &handlers,
                &states,
                &debug_flag,
            );
        }

        world
            .create_system(EventSystem {
                handlers: handlers.clone(),
                update: update.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(StateSystem {
                states: states.clone(),
                update: update.clone(),
                is_init: Cell::new(false),
            })
            .with_priority(1)
            .build();

        world
            .create_system(LayoutSystem {
                layout_objects: layout_objects.clone(),
                update: update.clone(),
            })
            .with_priority(2)
            .build();

        world
            .create_system(PostLayoutStateSystem {
                states: states.clone(),
                update: update.clone(),
            })
            .with_priority(3)
            .build();

        let update = Arc::new(AtomicBool::new(true));
        let running = Arc::new(AtomicBool::new(true));

        world
            .create_system(RenderSystem {
                update: update.clone(),
            })
            .with_priority(4)
            .build();

        world
            .create_system(RequestEventsSystem {
                running: running.clone(),
            })
            .with_priority(5)
            .build();

        // add window to global
        if let Ok(global) = world
            .entity_component_manager()
            .borrow_mut_component::<Global>(0)
        {
            global.window = Some(window);
            global.theme = self.theme;
        }

        self.application.main_window_runner = Some(Runner::new(Box::new(move || {
            world.run();

            running.load(atomic::Ordering::Acquire)
        })));
    }
}

// Builds the widget tree.
fn build_tree(
    window: &mut Box<traits::Window>,
    root: Template,
    world: &mut World<Tree>,
    // render_objects: &Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
    layout_objects: &Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,
    handlers: &Rc<RefCell<BTreeMap<Entity, Vec<Rc<EventHandler>>>>>,
    states: &Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
    debug_flag: &Rc<Cell<bool>>,
) {
    fn expand(
        window: &mut Box<traits::Window>,
        world: &mut World<Tree>,
        // render_objects: &Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
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
                .with(Bounds::default())
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
                .with(Bounds::default())
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
                // todo: rendewrobject builder ...
                if render_object == TypeId::of::<Rectangle>() {
                    window.insert_rectangle(entity as usize, Rectangle::default());
                }
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
                "{} (id = {}, children_lenght = {})",
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
                window,
                world,
                // render_objects,
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
        window,
        world,
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
