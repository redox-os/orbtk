use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::Arc;

use {
    Backend, Entity, EventSystem, LayoutObject, LayoutSystem, Rect, RenderObject, RenderSystem,
    Template, Theme, Tree, Widget, World, EventManager,
};

#[derive(Default)]
pub struct TreeManager {
    world: World<Tree>,
    backend: Option<Arc<RefCell<Backend>>>,
    render_objects: Arc<RefCell<HashMap<Entity, Box<RenderObject>>>>,
    layout_objects: Arc<RefCell<HashMap<Entity, Box<LayoutObject>>>>,
    window_size: Arc<Cell<(u32, u32)>>,
    event_manager: EventManager,
}

impl TreeManager {
    pub fn new(backend: Arc<RefCell<Backend>>, theme: Arc<Theme>) -> Self {
        let mut world = World::from_container(Tree::default());
        let render_objects = Arc::new(RefCell::new(HashMap::new()));
        let layout_objects = Arc::new(RefCell::new(HashMap::new()));
        let size = backend.borrow().size();
        let window_size = Arc::new(Cell::new(size));

        world
            .create_system(EventSystem {
                _backend: backend.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(LayoutSystem {
                theme: theme.clone(),
                layout_objects: layout_objects.clone(),
                window_size: window_size.clone(),
            })
            .with_priority(1)
            .build();

        world
            .create_system(RenderSystem {
                backend: backend.clone(),
                render_objects: render_objects.clone(),
            })
            .with_priority(2)
            .build();

        TreeManager {
            world,
            backend: Some(backend),
            render_objects,
            layout_objects,
            window_size: window_size,
            event_manager: EventManager::default(),
        }
    }

    pub fn root(&mut self, root: Arc<Widget>) {
        fn expand(
            world: &mut World<Tree>,
            render_objects: &Arc<RefCell<HashMap<Entity, Box<RenderObject>>>>,
            layout_objects: &Arc<RefCell<HashMap<Entity, Box<LayoutObject>>>>,
            widget: Arc<Widget>,
            parent: Entity,
        ) -> Entity {
            let entity = {
                let mut entity_builder = world.create_entity().with(Rect::default());

                for property in widget.all_properties() {
                    entity_builder = entity_builder.with_box(property);
                }

                let entity = entity_builder.build();

                if let Some(render_object) = widget.render_object() {
                    render_objects.borrow_mut().insert(entity, render_object);
                }

                layout_objects
                    .borrow_mut()
                    .insert(entity, widget.layout_object());

                entity
            };

            match widget.template() {
                Template::Single(child) => {
                    let child = expand(world, render_objects, layout_objects, child, parent);
                    let _result = world.entity_container().append_child(entity, child);
                }
                Template::Mutli(children) => {
                    for child in children {
                        let child = expand(world, render_objects, layout_objects, child, parent);
                        let _result = world.entity_container().append_child(entity, child);
                    }
                }
                _ => {}
            }

            entity
        }

        expand(
            &mut self.world,
            &self.render_objects,
            &self.layout_objects,
            root,
            0,
        );

        for node in self.world.entity_container().into_iter() {
            println!("Node: {}", node);
        }
    }

    pub fn run(&mut self) -> bool {
        let mut running = true;
        
        if let Some(backend) = &self.backend {
            self.window_size.set(backend.borrow().size());
        }

        self.world.run();

        if let Some(backend) = &self.backend {
            running = backend.borrow_mut().drain_events(&mut self.event_manager);
        }

        running
    }
}
