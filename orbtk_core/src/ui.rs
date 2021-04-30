use std::marker::*;

use legion::*;

use atomic_refcell::*;

use crate::*;

pub struct Ui<S>
where
    S: Default + Clone + PartialEq,
{
    world: World,
    tree: Option<Tree>,
    resources: Resources,
    state: S,
    view_builder: Option<Box<dyn Fn(&mut S) -> Node>>,
}

impl<S> Ui<S>
where
    S: Default + Clone + PartialEq,
{
    pub fn new(state: S) -> Self {
        Self {
            world: World::default(),
            tree: None,
            resources: Resources::default(),
            state,
            view_builder: None,
        }
    }

    fn layout(&mut self) {}

    fn draw_entity(&mut self, entity: Entity, rtx: &mut dyn RenderContext2D) {
        // if let Some(entry) = self.world.entry(entity) {
        //     if let Ok(render_component) = entry.get_component::<components::RenderComponent>() {
        //         render_component.draw(entry, rtx);
        //     }
        // }
    }

    fn draw(&mut self, rtx: &mut dyn RenderContext2D) {
        if self.tree.is_none() {
            return;
        }

        // if let Some(tree) = &self.tree {
        //     self.draw_entity(tree.root(), rtx);
        // }

        // for render in render_query.iter(&self.world) {
        //     render.draw(&self.world, rtx)
        // }

        // for bounds in render_query.iter(&self.world) {
        //     println!("{:?}", bounds);

        //     let mut test_query = <&components::TypeComponent>::query();

        //     for type_com in test_query.iter(&self.world) {}
        // }
    }

    fn event(&mut self) {}

    fn build_node(&mut self, mut node: Node, parent: Option<Entity>) {
        // move all components of the node in the world+ of the ui
        self.world.move_from(&mut node.world, &any());
        let entity = node.entity;

        // add the node to the tree
        if let Some(parent) = parent {
            if let Some(tree) = &mut self.tree {
                tree.push(entity, parent);
            }
        }

        // build all children of the node
        if let Some(children) = node.children {
            for child in children {
                self.build_node(child, Some(entity));
            }
        }
    }

    fn build(&mut self) {
        if let Some(view_builder) = &self.view_builder {
            self.world.clear();

            let root = view_builder(&mut self.state);

            self.tree = Some(Tree::new(root.entity));

            self.build_node(root, None);
        }
    }

    /// Inserts a global resource.
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(resource);
    }

    pub fn resource<T: 'static>(&self) -> Option<AtomicRef<'_, T>> {
        self.resources.get::<T>()
    }
    pub fn mut_resource<T: 'static>(&mut self) -> Option<AtomicRefMut<'_, T>> {
        self.resources.get_mut::<T>()
    }

    pub fn set_view<F>(&mut self, view_builder: F)
    where
        F: Fn(&mut S) -> Node + 'static,
    {
        self.view_builder = Some(Box::new(view_builder));
        self.build();
    }

    pub fn run(&mut self, rtx: &mut dyn RenderContext2D) {
        let old_state = self.state.clone();

        self.event();

        if self.state != old_state {
            self.build();
        }

        self.draw(rtx);
    }
}
