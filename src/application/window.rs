use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::collections::BTreeMap;

use dces::{ComponentBox, Component, Entity, World};

use application::Application;
use backend::{target_backend, BackendRunner};
use enums::ParentType;
use event::EventHandler;
use layout_object::{DefaultLayoutObject, LayoutObject};
use render_object::RenderObject;
use state::State;
use structs::{Point, Rect};
use systems::{EventSystem, LayoutSystem, RenderSystem, StateSystem};
use theme::Theme;
use tree::Tree;
use Global;

pub struct Template {
    children: Vec<Template>,
    parent_type: ParentType,
    state: Option<Box<State>>,
    event_handlers: Vec<Box<EventHandler>>,
    render_object: Option<Box<RenderObject>>,
    layout_object: Box<LayoutObject>,

    // todo: only one prop type per widget.
    properties: Vec<ComponentBox>,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            children: vec![],
            parent_type: ParentType::None,
            state: None,
            event_handlers: vec![],
            render_object: None,
            layout_object: Box::new(DefaultLayoutObject),
            properties: vec![],
        }
    }
}

impl Template {
    pub fn as_parent_type(mut self, parent_type: ParentType) -> Self {
        self.parent_type = parent_type;
        self
    }

    pub fn with_child(mut self, child: Template) -> Self {
        match self.parent_type {
            ParentType::Single => {
                self.children.clear();
                self.children.push(child);
            }
            ParentType::Multi => {
                self.children.push(child);
            }
            _ => return self,
        }

        self
    }

    pub fn with_state(mut self, state: Box<State>) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_event_handler(mut self, handler: Box<EventHandler>) -> Self {
        self.event_handlers.push(handler);
        self
    }

    pub fn with_render_object(mut self, render_object: Box<RenderObject>) -> Self {
        self.render_object = Some(render_object);
        self
    }

    pub fn with_layout_object(mut self, layout_object: Box<LayoutObject>) -> Self {
        self.layout_object = layout_object;
        self
    }

    // todo shared component

    pub fn with_property<C: Component>(mut self, property: C) -> Self {
        self.properties.push(ComponentBox::new::<C>(property));
        self
    }
}

pub struct Window {
    pub backend_runner: Box<BackendRunner>,

    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,

    pub layout_objects: Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,

    pub root: Option<Template>,

    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<EventHandler>>>>>,

    pub states: Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,

    pub update: Rc<Cell<bool>>,

    pub debug_flag: Rc<Cell<bool>>,
}

impl Window {
    pub fn run(&mut self) {
        self.backend_runner.run(self.update.clone());
    }
}

pub struct WindowBuilder<'a> {
    pub application: &'a mut Application,
    pub bounds: Rect,
    pub title: String,
    pub theme: Theme,
    pub root: Option<Template>,
    pub debug_flag: bool,
}

impl<'a> WindowBuilder<'a> {
    pub fn with_bounds(mut self, bounds: Rect) -> Self {
        self.bounds = bounds;
        self
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_root(mut self, root: Template) -> Self {
        self.root = Some(root);
        self
    }

    pub fn with_debug_flag(mut self, debug: bool) -> Self {
        self.debug_flag = debug;
        self
    }

    pub fn build(mut self) {
        let (mut runner, backend) = target_backend(&self.title, self.bounds, self.theme);
        let mut world = World::from_container(Tree::default());
        let render_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let layout_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));
        let update = Rc::new(Cell::new(true));
        let debug_flag = Rc::new(Cell::new(self.debug_flag));

        if let Some(root) = &mut self.root {
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
            root: self.root,
            handlers,
            states,
            update,
            debug_flag,
        })
    }
}

fn build_tree(
    root: &mut Template,
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
        template: &mut Template,
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
                .insert(root, Box::new(DefaultLayoutObject));
        }

        let entity = {
            let mut entity_builder = world
                .create_entity()
                .with(Rect::default())
                .with(Point::default());

            // todo: shared properties / components
            // for propety in template.properties {

            // }

            // for property in widget.all_properties() {
            //     match property {
            //         PropertyResult::Property(property, source) => {
            //             entity_builder = entity_builder.with_box(property);
            //             source.set(Some(entity_builder.entity));
            //         }
            //         PropertyResult::Source(source) => {
            //             entity_builder = entity_builder.with_shared_box(source);
            //         }
            //         PropertyResult::PropertyNotFound => {}
            //     }
            // }

            let entity = entity_builder.build();

            // if let Some(render_object) = widget.render_object() {
            //     render_objects.borrow_mut().insert(entity, render_object);
            // }

            // let widget_handlers = widget.event_handlers();

            // if ! widget_handlers.is_empty() {
            //     let mut event_handlers = vec![];

            //     for handler in widget_handlers {
            //         event_handlers.push(handler.clone());
            //     }

            //     handlers.borrow_mut().insert(entity, event_handlers);
            // }

            // if let Some(state) = widget.state() {
            //     states.borrow_mut().insert(entity, state.clone());
            // }

            // layout_objects
            //     .borrow_mut()
            //     .insert(entity, widget.layout_object());

            entity
        };

        if world.entity_container().len() == 2 {
            let root = world.entity_container().root;
            let _result = world.entity_container().append_child(root, entity);
        }

        // match widget.template() {
        //     Template::Single(child) => {
        //         if debug_flag.get() {
        //             println!("Node ID: {}", entity);
        //         }
        //         let child = expand(
        //             world,
        //             render_objects,
        //             layout_objects,
        //             handlers,
        //             states,
        //             &child,
        //             debug_flag,
        //         );
        //         let _result = world.entity_container().append_child(entity, child);
        //     }
        //     Template::Mutli(children) => {
        //         if debug_flag.get() {
        //             println!("Node ID: {}", entity);
        //         }
        //         for child in children {
        //             let child = expand(
        //                 world,
        //                 render_objects,
        //                 layout_objects,
        //                 handlers,
        //                 states,
        //                 &child,
        //                 debug_flag,
        //             );
        //             let _result = world.entity_container().append_child(entity, child);
        //         }
        //     }
        //     _ => {
        //         if debug_flag.get() {
        //             println!("Node ID: {}", entity);
        //         }
        //     }
        // }

        entity
    }

    if debug_flag.get() {
        println!("------ Start build tree ------\n");
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
