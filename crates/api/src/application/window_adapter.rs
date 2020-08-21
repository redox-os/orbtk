use std::{cell::RefCell, collections::HashMap, sync::mpsc};

use dces::prelude::*;

use crate::{
    application::*,
    event::*,
    properties::*,
    render,
    services::{Clipboard, Settings},
    shell,
    shell::{ShellRequest, WindowRequest, WindowSettings},
    systems::*,
    tree::Tree,
    utils::{Point, Rectangle},
    widget_base::*,
};

/// Represents a window. Each window has its own tree, event pipeline and shell.
pub struct WindowAdapter {
    world: World<Tree, StringComponentStore, render::RenderContext2D>,
    ctx: ContextProvider,
}

impl WindowAdapter {
    /// Creates a new WindowAdapter.
    pub fn new(
        world: World<Tree, StringComponentStore, render::RenderContext2D>,
        ctx: ContextProvider,
    ) -> Self {
        WindowAdapter { world, ctx }
    }
}

impl WindowAdapter {
    fn root(&mut self) -> Entity {
        self.world
            .entity_component_manager()
            .entity_store()
            .root
            .unwrap()
    }
}

impl shell::WindowAdapter for WindowAdapter {
    fn resize(&mut self, width: f64, height: f64) {
        let root = self.root();
        self.ctx
            .event_queue
            .borrow_mut()
            .register_event_with_strategy(
                WindowEvent::Resize { width, height },
                EventStrategy::Direct,
                root,
            );
    }

    fn mouse(&mut self, x: f64, y: f64) {
        let root = self.root();
        self.ctx.mouse_position.set(Point::new(x, y));
        self.ctx.event_queue.borrow_mut().register_event(
            MouseMoveEvent {
                position: Point::new(x, y),
            },
            root,
        )
    }

    fn scroll(&mut self, delta_x: f64, delta_y: f64) {
        let root = self.root();
        self.ctx.event_queue.borrow_mut().register_event(
            ScrollEvent {
                delta: Point::new(delta_x, delta_y),
            },
            root,
        )
    }

    fn mouse_event(&mut self, event: shell::MouseEvent) {
        let root = self.root();
        match event.state {
            shell::ButtonState::Up => {
                self.ctx.event_queue.borrow_mut().register_event(
                    MouseUpEvent {
                        position: event.position,
                        button: event.button,
                    },
                    root,
                );
                self.ctx.event_queue.borrow_mut().register_event(
                    GlobalMouseUpEvent {
                        position: event.position,
                        button: event.button,
                    },
                    root,
                );
            }
            shell::ButtonState::Down => self.ctx.event_queue.borrow_mut().register_event(
                MouseDownEvent {
                    position: event.position,
                    button: event.button,
                },
                root,
            ),
        }
    }

    fn mouse_position(&self) -> Point {
        self.ctx.mouse_position.get()
    }

    fn key_event(&mut self, event: shell::KeyEvent) {
        let root = self.root();
        match event.state {
            shell::ButtonState::Up => self
                .ctx
                .event_queue
                .borrow_mut()
                .register_event(KeyUpEvent { event }, root),
            shell::ButtonState::Down => {
                self.ctx
                    .event_queue
                    .borrow_mut()
                    .register_event(KeyDownEvent { event }, root);
            }
        }
    }

    fn quit_event(&mut self) {
        let root = self.root();

        self.ctx
            .event_queue
            .borrow_mut()
            .register_event(SystemEvent::Quit, root);
    }

    fn active(&mut self, active: bool) {
        let root = self.root();

        self.ctx
            .event_queue
            .borrow_mut()
            .register_event_with_strategy(
                WindowEvent::ActiveChanged(active),
                EventStrategy::Direct,
                root,
            );
    }

    fn run(&mut self, render_context: &mut render::RenderContext2D) {
        self.world.run_with_context(render_context);
    }

    fn file_drop_event(&mut self, file_name: String) {
        let root = self.root();
        self.ctx.event_queue.borrow_mut().register_event(
            DropFileEvent {
                file_name,
                position: self.mouse_position(),
            },
            root,
        );
    }

    fn text_drop_event(&mut self, text: String) {
        let root = self.root();
        self.ctx.event_queue.borrow_mut().register_event(
            DropTextEvent {
                text,
                position: self.ctx.mouse_position.get(),
            },
            root,
        );
    }
}

/// Creates a `WindowAdapter` and a `WindowSettings` object from a window builder closure.
pub fn create_window<F: Fn(&mut BuildContext) -> Entity + 'static>(
    app_name: impl Into<String>,
    theme: Theme,
    request_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
    create_fn: F,
) -> (WindowAdapter, WindowSettings, mpsc::Receiver<WindowRequest>) {
    let app_name = app_name.into();
    let mut world: World<Tree, StringComponentStore, render::RenderContext2D> =
        World::from_stores(Tree::default(), StringComponentStore::default());

    let (sender, receiver) = mpsc::channel();

    let registry = Rc::new(RefCell::new(Registry::new()));

    if app_name.is_empty() {
        registry
            .borrow_mut()
            .register("settings", Settings::default());
    } else {
        registry
            .borrow_mut()
            .register("settings", Settings::new(app_name.clone()));
    };

    registry
        .borrow_mut()
        .register("clipboard", Clipboard::new());

    let context_provider = ContextProvider::new(sender, request_sender, app_name);

    let window = {
        let overlay = Overlay::new().build(&mut BuildContext::new(
            world.entity_component_manager(),
            &context_provider.render_objects,
            &context_provider.layouts,
            &context_provider.handler_map,
            &mut *context_provider.states.borrow_mut(),
            &theme,
            &context_provider.event_queue,
        ));

        {
            let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
            tree.set_overlay(overlay);
        }

        let window = create_fn(&mut BuildContext::new(
            world.entity_component_manager(),
            &context_provider.render_objects,
            &context_provider.layouts,
            &context_provider.handler_map,
            &mut *context_provider.states.borrow_mut(),
            &theme,
            &context_provider.event_queue,
        ));

        {
            let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
            tree.set_root(window);
        }

        window
    };

    let constraint = *world
        .entity_component_manager()
        .component_store()
        .get::<Constraint>("constraint", window)
        .unwrap();

    let position = *world
        .entity_component_manager()
        .component_store()
        .get::<Point>("position", window)
        .unwrap();

    let mut fonts = HashMap::new();
    fonts.insert(
        "Roboto-Regular".to_string(),
        crate::theme::fonts::ROBOTO_REGULAR_FONT,
    );
    fonts.insert(
        "Roboto-Medium".to_string(),
        crate::theme::fonts::ROBOTO_MEDIUM_FONT,
    );
    fonts.insert(
        "MaterialIcons-Regular".to_string(),
        crate::theme::fonts::MATERIAL_ICONS_FONT,
    );

    let settings = WindowSettings {
        title: world
            .entity_component_manager()
            .component_store()
            .get::<String>("title", window)
            .unwrap()
            .clone(),
        borderless: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("borderless", window)
            .unwrap(),
        resizeable: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("resizeable", window)
            .unwrap(),
        always_on_top: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("always_on_top", window)
            .unwrap(),
        position: (position.x(), position.y()),
        size: (constraint.width(), constraint.height()),
        fonts,
    };

    let mut global = Global::default();
    global.theme = theme;

    world
        .entity_component_manager()
        .component_store_mut()
        .register("global", window, global);
    world
        .entity_component_manager()
        .component_store_mut()
        .register(
            "bounds",
            window,
            Rectangle::from((0.0, 0.0, constraint.width(), constraint.height())),
        );

    world.register_init_system(InitSystem::new(context_provider.clone(), registry.clone()));

    world.register_cleanup_system(CleanupSystem::new(
        context_provider.clone(),
        registry.clone(),
    ));

    world
        .create_system(EventStateSystem::new(
            context_provider.clone(),
            registry.clone(),
        ))
        .with_priority(0)
        .build();

    world
        .create_system(LayoutSystem::new(context_provider.clone()))
        .with_priority(1)
        .build();

    world
        .create_system(PostLayoutStateSystem::new(
            context_provider.clone(),
            registry,
        ))
        .with_priority(2)
        .build();

    world
        .create_system(RenderSystem::new(context_provider.clone()))
        .with_priority(3)
        .build();

    (
        WindowAdapter::new(world, context_provider),
        settings,
        receiver,
    )
}
