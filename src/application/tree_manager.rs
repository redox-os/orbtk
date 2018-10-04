use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::{mpsc::Receiver, mpsc::Sender, Arc};
use std::thread;

use {
    Entity, EventManager, LayoutObject, LayoutSystem, MouseEvent, Rect, RenderContainer,
    RenderObject, RenderSystem, SystemEvent, Template, Theme, Tree, Widget, WindowEvent, World,
};

#[derive(Default)]
pub struct TreeManager {
    world: World<Tree>,
    // backend: Option<Arc<Backend>>,
    render_objects: Arc<RefCell<HashMap<Entity, Arc<RenderObject>>>>,
    layout_objects: Arc<RefCell<HashMap<Entity, Box<LayoutObject>>>>,
    window_size: Arc<Cell<(u32, u32)>>,
    _event_manager: EventManager,
    running: bool,
    event_receiver: Option<Receiver<EventManager>>,
}

impl TreeManager {
    pub fn new(
        theme: Arc<Theme>,
        root: Option<Arc<Widget>>,
        event_receiver: Option<Receiver<EventManager>>,
        render_sender: Sender<Vec<RenderContainer>>,
        window_size: (u32, u32),
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut world = World::from_container(Tree::default());
            let render_objects = Arc::new(RefCell::new(HashMap::new()));
            let layout_objects = Arc::new(RefCell::new(HashMap::new()));
            // let size = backend.borrow().size();
            let window_size = Arc::new(Cell::new(window_size));

            // world
            //     .create_system(EventSystem {
            //         _backend: backend.clone(),
            //     })
            //     .with_priority(0)
            //     .build();

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
                    render_objects: render_objects.clone(),
                    render_sender,
                })
                .with_priority(2)
                .build();

            let mut tree_manager = TreeManager {
                world,
                render_objects,
                layout_objects,
                window_size,
                _event_manager: EventManager::default(),
                running: false,
                event_receiver,
            };

            if let Some(root) = root {
                tree_manager.root(root);
            }

            tree_manager.run();
        })
    }

    pub fn root(&mut self, root: Arc<Widget>) {
        fn expand(
            world: &mut World<Tree>,
            render_objects: &Arc<RefCell<HashMap<Entity, Arc<RenderObject>>>>,
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

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            //    println!("Events");

            if let Some(events) = &self.event_receiver {
                let mut events = events.try_recv();

                if let Ok(events) = &mut events {
                    while let Some(event) = events.dequeue() {
                        if event.is_type::<MouseEvent>() {
                            match &*event.downcast::<MouseEvent>().unwrap() {
                                MouseEvent::Move(mouse) => {
                                    println!("{}, {}", mouse.0, mouse.1);
                                }
                                MouseEvent::Down(_mouse) => {
                                    println!("Down");
                                }
                                MouseEvent::Up(_mouse) => {
                                    println!("Up");
                                }
                            }
                        } else if event.is_type::<SystemEvent>() {
                            match &*event.downcast::<SystemEvent>().unwrap() {
                                SystemEvent::Quit => {
                                    self.running = false;
                                }
                            }
                        } else if event.is_type::<WindowEvent>() {
                            match &*event.downcast::<WindowEvent>().unwrap() {
                                WindowEvent::Resize(size) => {
                                    self.window_size.set(*size);
                                }
                            }
                        }
                    }
                }
            }

            self.world.run();
        }
    }
}
