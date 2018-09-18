use std::cell::RefCell;
use std::sync::Arc;

use {Application, Backend, Rect, Theme, Widget, WidgetManager};

pub struct Window {
    pub widget_manager: WidgetManager,
    pub bounds: Rect,
    pub title: String,
    pub theme: Arc<Theme>,
    pub running: bool,
}

impl Window {
    pub fn run(&mut self) {
        'event: while self.running {
            self.widget_manager.run();
        }
    }
}

pub struct WindowBuilder<'a> {
    pub application: &'a mut Application,
    pub bounds: Rect,
    pub title: String,
    pub theme: Arc<Theme>,
    pub root: Option<Arc<Widget>>,
    pub renderer: Box<Backend>,
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

    pub fn with_renderer(mut self, renderer: Box<Backend>) -> Self {
        self.renderer = renderer;
        self
    }

    pub fn build(mut self) {
        self.renderer.bounds(&self.bounds);
        let mut widget_manager = WidgetManager::new(RefCell::new(self.renderer));

        if let Some(root) = self.root {
            widget_manager.root(root.clone());
        }

        let theme = self.theme.clone();
        self.application.windows.push(Window {
            widget_manager,
            bounds: self.bounds,
            title: self.title,
            theme,
            running: true,
        })
    }
}
