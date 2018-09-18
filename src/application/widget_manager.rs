use std::cell::RefCell;
use std::sync::Arc;

use dces::World;

use {Backend, Drawable, Rect, RenderSystem, Template, Widget};

#[derive(Default)]
pub struct WidgetManager {
    world: World,
}

impl WidgetManager {
    pub fn new(renderer: RefCell<Box<Backend>>) -> Self {
        let mut world = World::new();
        world
            .create_system(RenderSystem { renderer })
            .with_priority(0)
            .with_filter(|comp| {
                for co in comp {
                    if let Some(_) = co.downcast_ref::<Drawable>() {
                        return true;
                    }
                }
                false
            }).build();

        WidgetManager { world }
    }

    pub fn root(&mut self, root: Arc<Widget>) {
        let mut widgets = vec![];
        self.expand(root, &mut widgets);

        for widget in widgets {
            let mut entity_builder = self.world.create_entity();

            for component in widget.components() {
                entity_builder = entity_builder.with_box(component);
            }

            // add bounds
            entity_builder.with(Rect::new(10, 10, 200, 50)).build();
        }
    }

    fn expand(&mut self, widget: Arc<Widget>, widgets: &mut Vec<Arc<Widget>>) {
        match widget.template() {
            Template::Empty => {
                widgets.push(widget);
                return;
            }
            Template::Single(child) => {
                self.expand(child, widgets);
            }
            Template::Mutli(children) => {
                for child in children {
                    self.expand(child, widgets);
                }
            }
        }

        widgets.push(widget);
    }

    pub fn run(&mut self) {
        self.world.apply_filter_and_sort();
        self.world.run();
    }
}
