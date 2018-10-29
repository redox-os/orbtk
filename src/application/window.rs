use std::cell::RefCell;
use std::rc::Rc;

use std::collections::BTreeMap;

use dces::{Entity, World};

use application::Application;
use backend::{target_backend, BackendRunner};
use event::Handler;
use layout_object::LayoutObject;
use render_object::RenderObject;
use state::State;
use structs::{Point, Rect};
use systems::{EventSystem, LayoutSystem, RenderSystem, StateSystem};
use theme::Theme;
use tree::Tree;
use widget::{PropertyResult, Template, Widget};

pub struct Window {
    pub backend_runner: Box<BackendRunner>,

    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,

    pub layout_objects: Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,

    pub root: Option<Rc<Widget>>,

    pub handlers: Rc<RefCell<BTreeMap<Entity, Rc<Handler>>>>,

    pub states: Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
}

impl Window {
    pub fn run(&mut self) {
        self.backend_runner.run();
    }
}

pub struct WindowBuilder<'a> {
    pub application: &'a mut Application,
    pub bounds: Rect,
    pub title: String,
    pub theme: Theme,
    pub root: Option<Rc<Widget>>,
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

    pub fn with_root<W: Widget>(mut self, root: W) -> Self {
        self.root = Some(Rc::new(root));
        self
    }

    pub fn build(self) {
        let (mut runner, backend) = target_backend(&self.title, self.bounds, self.theme);
        let mut world = World::from_container(Tree::default());
        let render_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let layout_objects = Rc::new(RefCell::new(BTreeMap::new()));
        let handlers = Rc::new(RefCell::new(BTreeMap::new()));
        let states = Rc::new(RefCell::new(BTreeMap::new()));

        if let Some(root) = &self.root {
            build_tree(
                root,
                &mut world,
                &render_objects,
                &layout_objects,
                &handlers,
                &states,
            );
        }

        world
            .create_system(EventSystem {
                backend: backend.clone(),
                handlers: handlers.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(StateSystem {
                states: states.clone(),
            })
            .with_priority(1)
            .build();

        world
            .create_system(LayoutSystem {
                backend: backend.clone(),
                layout_objects: layout_objects.clone(),
            })
            .with_priority(2)
            .build();

        world
            .create_system(RenderSystem {
                backend: backend.clone(),
                render_objects: render_objects.clone(),
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
        })
    }
}

fn build_tree(
    root: &Rc<Widget>,
    world: &mut World<Tree>,
    render_objects: &Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
    layout_objects: &Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,
    handlers: &Rc<RefCell<BTreeMap<Entity, Rc<Handler>>>>,
    states: &Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
) {
    fn expand(
        world: &mut World<Tree>,
        render_objects: &Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
        layout_objects: &Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,
        handlers: &Rc<RefCell<BTreeMap<Entity, Rc<Handler>>>>,
        states: &Rc<RefCell<BTreeMap<Entity, Rc<State>>>>,
        widget: &Rc<Widget>,
        parent: Entity,
    ) -> Entity {
        let entity = {
            let mut entity_builder = world
                .create_entity()
                .with(Rect::default())
                .with(Point::default());

            for property in widget.all_properties() {
                match property {
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

            if let Some(render_object) = widget.render_object() {
                render_objects.borrow_mut().insert(entity, render_object);
            }

            if let Some(handler) = widget.handler() {
                handlers.borrow_mut().insert(entity, handler.clone());
            }

            if let Some(state) = widget.state() {
                states.borrow_mut().insert(entity, state.clone());
            }

            layout_objects
                .borrow_mut()
                .insert(entity, widget.layout_object());

            entity
        };

        match widget.template() {
            Template::Single(child) => {
                let child = expand(
                    world,
                    render_objects,
                    layout_objects,
                    handlers,
                    states,
                    &child,
                    parent,
                );
                let _result = world.entity_container().append_child(entity, child);
            }
            Template::Mutli(children) => {
                for child in children {
                    let child = expand(
                        world,
                        render_objects,
                        layout_objects,
                        handlers,
                        states,
                        &child,
                        parent,
                    );
                    let _result = world.entity_container().append_child(entity, child);
                }
            }
            _ => {}
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
        0,
    );

    // let mut nodes = vec![];

    // for node in world.entity_container().into_iter() {
    //     nodes.push(node);
    // }

    // for node in nodes {
    //     let mut class = "";
    //     if let Ok(ok) =  world.entity_component_manager.borrow_component::<Selector>(node) {

    //     }
    //     println!("Node: {}", node);
    // }
}
