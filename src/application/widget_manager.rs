use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use {
    Backend, Entity, LayoutObject, LayoutSystem, Rect, RenderObject, RenderSystem,
    Template, Theme, Tree, Widget, World,
};

pub struct EntityId(u32);

#[derive(Default)]
pub struct WidgetManager {
    world: World,
    entity_counter: u32,
    tree: Arc<RefCell<Tree>>,
    backend: Option<Arc<RefCell<Backend>>>,
    render_objects: Arc<RefCell<HashMap<Entity, Box<RenderObject>>>>,
    layout_objects: Arc<RefCell<HashMap<Entity, Box<LayoutObject>>>>,
}

impl WidgetManager {
    pub fn new(backend: Arc<RefCell<Backend>>, theme: Arc<Theme>) -> Self {
        let mut world = World::new();
        let tree = Arc::new(RefCell::new(Tree::default()));
        let render_objects = Arc::new(RefCell::new(HashMap::new()));
        let layout_objects = Arc::new(RefCell::new(HashMap::new()));

        world
            .create_system(LayoutSystem {
                tree: tree.clone(),
                theme: theme.clone(),
                layout_objects: layout_objects.clone(),
            })
            .with_priority(0)
            .build();

        world
            .create_system(RenderSystem {
                tree: tree.clone(),
                backend: backend.clone(),
                render_objects: render_objects.clone(),
            })
            .with_priority(1)
            .with_sort(|comp_a, comp_b| {
                let id_a;
                let id_b;

                if let Some(id) = comp_a.downcast_ref::<EntityId>() {
                    id_a = id;
                } else {
                    return None;
                }

                if let Some(id) = comp_b.downcast_ref::<EntityId>() {
                    id_b = id;
                } else {
                    return None;
                }

                Some(id_a.0.cmp(&id_b.0))
            })
            .build();

        //  ).with_filter(|comp| {
        //     for co in comp {
        //         if let Some(_) = co.downcast_ref::<Drawable>() {
        //             return true;
        //         }
        //     }
        //     false
        // }

        WidgetManager {
            world,
            entity_counter: 0,
            tree,
            backend: Some(backend),
            render_objects,
            layout_objects,
        }
    }

    pub fn root(&mut self, root: Arc<Widget>) {
        fn expand(
            world: &mut World,
            tree: &Arc<RefCell<Tree>>,
            render_objects: &Arc<RefCell<HashMap<Entity, Box<RenderObject>>>>,
            layout_objects: &Arc<RefCell<HashMap<Entity, Box<LayoutObject>>>>,
            widget: Arc<Widget>,
            parent: Entity,
            entity_counter: &mut u32,
        ) -> Entity {
            let entity = {
                // add bounds and default layout

                // todo: find better place for default components
                let mut entity_builder = world
                    .create_entity()
                    .with(Rect::new(0, 0, 200, 50))
                    .with(EntityId(*entity_counter));

                *entity_counter += 1;

                for property in widget.all_properties() {
                    entity_builder = entity_builder.with_box(property);
                }

                let entity = entity_builder.build();

                // todo: use one render / layout object per widget type
                if let Some(render_object) = widget.render_object() {
                    render_objects.borrow_mut().insert(entity, render_object);
                }

                layout_objects
                    .borrow_mut()
                    .insert(entity, widget.layout_object());

                tree.borrow_mut().register_node(entity);

                entity
            };

            match widget.template() {
                Template::Single(child) => {
                    let child = expand(
                        world,
                        tree,
                        render_objects,
                        layout_objects,
                        child,
                        parent,
                        entity_counter,
                    );
                    let _result = tree.borrow_mut().append_child(entity, child);
                }
                Template::Mutli(children) => {
                    for child in children {
                        let child = expand(
                            world,
                            tree,
                            render_objects,
                            layout_objects,
                            child,
                            parent,
                            entity_counter,
                        );
                        let _result = tree.borrow_mut().append_child(entity, child);
                    }
                }
                _ => {}
            }

            entity
        }

        expand(
            &mut self.world,
            &self.tree,
            &self.render_objects,
            &self.layout_objects,
            root,
            0,
            &mut self.entity_counter,
        );
    }

    pub fn run(&mut self) {
        self.world.apply_filter_and_sort();
        self.world.run();

        if let Some(backend) = &self.backend {
            backend.borrow_mut().update();
        }
    }
}
