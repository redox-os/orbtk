#![crate_name = "orbtk"]
#![crate_type = "lib"]
#![deny(warnings)]
#![feature(const_fn)]

extern crate dces;

pub use dces::prelude::*;
use std::any::Any;
use std::sync::Arc;

extern crate cssparser;
extern crate orbclient;
extern crate orbimage;
#[macro_use]
extern crate lazy_static;

pub use orbclient::color::Color;
pub use orbclient::renderer::Renderer;

pub use cell::CloneCell;
// pub use drawable::*;
// pub use event::Event;
// pub use layouts::*;
pub use structs::*;
pub use theme::{Selector, Theme};
// pub use traits::*;
// pub use tree::*;
// pub use window::{InnerWindow, Window, Application};
// pub use widgets::*;

pub mod cell;
// pub mod drawable;
// pub mod event;
// pub mod layouts;
pub mod structs;
// pub mod traits;
// pub mod window;
// pub mod draw;
pub mod theme;
// pub mod tree;
// pub mod widgets;

struct RenderSystem;

pub struct Drawable {
    draw_fn: Box<Fn(&Selector)>,
}

impl Drawable {
    pub fn new(draw_fn: Box<Fn(&Selector)>) -> Self {
        Drawable { draw_fn }
    }

    pub fn draw(&self, selector: &Selector) {
        (self.draw_fn)(selector)
    }
}

impl System for RenderSystem {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        for entity in entities {
            if let Ok(drawable) = ecm.borrow_component::<Drawable>(*entity) {
                if let Ok(selector) = ecm.borrow_component::<Selector>(*entity) {
                    drawable.draw(selector);
                }
            }

            if let Ok(selector) = ecm.borrow_component::<Selector>(*entity) {
                println!("{:?}", selector);
            } else {
                println!("No {}", entity);
            }
        }
    }
}

pub enum Template {
    Empty,
    Single(Arc<Widget>),
    Mutli(Vec<Arc<Widget>>),
}

pub trait Widget: Any {
    fn template(&self) -> Template {
        Template::Empty
    }
    fn components(&self) -> Vec<ComponentBox> {
        vec![]
    }
}

#[derive(Default)]
pub struct Border {
    child: Option<Arc<Widget>>,
}

impl Border {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn child(&mut self, child: Arc<Widget>) {
        self.child = Some(child);
    }
}

impl Widget for Border {
    fn template(&self) -> Template {
        if let Some(child) = &self.child {
            Template::Single(child.clone())
        } else {
            Template::Empty
        }
    }

    fn components(&self) -> Vec<ComponentBox> {
        vec![ComponentBox::new(Drawable::new(Box::new(
            |_selector: &Selector| println!("Draw border"),
        )))]
    }
}

pub struct Label {
    pub selector: ComponentBox,
}

impl Label {
    pub fn new(selector: Selector) -> Self {
        Label {
            selector: ComponentBox::new(selector),
        }
    }
}

impl Widget for Label {
    fn components(&self) -> Vec<ComponentBox> {
        vec![]
    }
}

pub struct Button;

impl Widget for Button {
    fn template(&self) -> Template {
        Template::Single(Arc::new(Border::new()))
    }

    fn components(&self) -> Vec<ComponentBox> {
        vec![ComponentBox::new(Selector::new(Some("button")))]
    }
}

#[derive(Default)]
pub struct WidgetManager {
    world: World,
}

impl WidgetManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn root(&mut self, root: Arc<Widget>) {
        let mut widgets = vec![];
        self.expand(root, &mut widgets);

        self.world
            .create_system(RenderSystem)
            .with_priority(0)
            .with_filter(|comp| {
                for co in comp {
                    if let Some(_) = co.downcast_ref::<Drawable>() {
                        return true;
                    }
                }
                false
            }).build();

        for widget in widgets {
            let mut entity_builder = self.world.create_entity();

            for component in widget.components() {
                entity_builder = entity_builder.with_box(component);
            }

            // add bounds
            entity_builder.with(Rect::new(0, 0, 200, 50)).build();
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

pub struct Application {
    // list of windows
    // theme
    theme: Arc<Theme>,
    windows: Vec<Window>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            theme: Arc::new(Theme::new()),
            windows: vec![],
        }
    }

    pub fn create_window(&mut self) -> WindowBuilder {
        let theme = self.theme.clone();
        WindowBuilder {
            application: self,
            bounds: Rect::default(),
            title: String::from(""),
            theme,
            root: None,
        }
    }

    pub fn run(&mut self) {
        for window in &mut self.windows {
            window.run();
        }
    }
}

pub struct WindowBuilder<'a> {
    pub application: &'a mut Application,
    pub bounds: Rect,
    pub title: String,
    pub theme: Arc<Theme>,
    pub root: Option<Arc<Widget>>,
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
        self.theme = Arc::new(theme);
        self
    }

    pub fn with_root<W: Widget>(mut self, root: W) -> Self {
        self.root = Some(Arc::new(root));
        self
    }

    pub fn build(self) {
        let mut widget_manager = WidgetManager::new();

        if let Some(root) = self.root {
            widget_manager.root(root.clone());
        }

        let theme = self.theme.clone();
        self.application.windows.push(Window {
            widget_manager,
            bounds: self.bounds,
            title: self.title,
            theme,
        })
    }
}

pub struct Window {
    pub widget_manager: WidgetManager,
    pub bounds: Rect,
    pub title: String,
    pub theme: Arc<Theme>,
    // size
}

impl Window {
    pub fn run(&mut self) {
        self.widget_manager.run();
    }
}
