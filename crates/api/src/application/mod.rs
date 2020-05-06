//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use std::{cell::RefCell, rc::Rc};

use dces::prelude::{Entity, World};

use crate::{
    prelude::*,
    render::RenderContext2D,
    shell::{Shell, ShellBuilder},
    tree::*,
    utils::{Point, Rectangle},
};

pub use self::context_provider::*;
pub use self::global::*;
pub use self::overlay::*;
pub use self::shell_adapter::*;

mod context_provider;
mod global;
mod overlay;
mod shell_adapter;

/// The `Application` represents the entry point of an OrbTk based application.
pub struct Application {
    shells: Vec<Shell<ShellAdapter>>,
    name: Box<str>,
}

impl Application {
    /// Creates a new application.
    pub fn new() -> Self {
        Application::from_name("orbtk_application")
    }

    /// Create a new application with the given name.
    pub fn from_name(name: impl Into<Box<str>>) -> Self {
        Application {
            name: name.into(),
            shells: vec![],
        }
    }

    /// Creates a new window and add it to the application.
    pub fn window<F: Fn(&mut BuildContext) -> Entity + 'static>(mut self, create_fn: F) -> Self {
        let mut world: World<Tree, StringComponentStore, RenderContext2D> =
            World::from_stores(Tree::default(), StringComponentStore::default());

        let registry = Rc::new(RefCell::new(Registry::new()));
        let context_provider = ContextProvider::new();

        let theme = crate::theme::default_theme();

        let window = {
            let overlay = Overlay::create().build(&mut BuildContext::new(
                world.entity_component_manager(),
                &context_provider.render_objects,
                &context_provider.layouts,
                &context_provider.handler_map,
                &mut *context_provider.states.borrow_mut(),
                &theme,
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
            ));

            {
                let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
                tree.set_root(window);
            }

            window
        };

        

        let title = world
            .entity_component_manager()
            .component_store()
            .get::<String>("title", window)
            .unwrap()
            .clone();
        let borderless = *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("borderless", window)
            .unwrap();
        let resizeable = *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("resizeable", window)
            .unwrap();
        let always_on_top = *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("always_on_top", window)
            .unwrap();
        let position = *world
            .entity_component_manager()
            .component_store()
            .get::<Point>("position", window)
            .unwrap();
        let constraint = *world
            .entity_component_manager()
            .component_store()
            .get::<Constraint>("constraint", window)
            .unwrap();

        world
            .entity_component_manager()
            .component_store_mut()
            .register("global", window, Global::default());
        world
            .entity_component_manager()
            .component_store_mut()
            .register("global", window, Global::default());
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
                registry.clone(),
            ))
            .with_priority(2)
            .build();

        world
            .create_system(RenderSystem::new(
                context_provider.clone()
            ))
            .with_priority(3)
            .build();

        let mut shell = ShellBuilder::new(ShellAdapter::new(world, context_provider))
            .title(&(title)[..])
            .bounds(Rectangle::from((
                position.x,
                position.y,
                constraint.width(),
                constraint.height(),
            )))
            .borderless(borderless)
            .resizeable(resizeable)
            .always_on_top(always_on_top)
            .build();

        #[cfg(not(target_arch = "wasm32"))]
        shell
            .render_context_2_d()
            .register_font("Roboto Regular", crate::theme::fonts::ROBOTO_REGULAR_FONT);

        #[cfg(not(target_arch = "wasm32"))]
        shell
            .render_context_2_d()
            .register_font("Roboto Medium", crate::theme::fonts::ROBOTO_MEDIUM_FONT);

        #[cfg(not(target_arch = "wasm32"))]
        shell.render_context_2_d().register_font(
            "Material Icons",
            crate::theme::fonts::MATERIAL_ICONS_REGULAR_FONT,
        );

        self.shells.push(shell);

        self
    }

    /// Starts the application and run it until quit is requested.
    pub fn run(mut self) {
        while let Some(mut shell) = self.shells.pop() {
            shell.run();
        }
    }
}
