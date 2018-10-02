use std::cell::RefCell;
use std::sync::Arc;

use {Application, Backend, Rect, Theme, Widget, TreeManager};

pub struct Window {
    pub tree_manager: TreeManager,
    pub bounds: Rect,
    pub title: String,
    pub theme: Arc<Theme>,
    pub running: bool,
}

impl Window {
    pub fn run(&mut self) {
        loop {
            if !self.tree_manager.run() {
                break;
            }
        }
    }
}

pub struct WindowBuilder<'a> {
    pub application: &'a mut Application,
    pub bounds: Rect,
    pub title: String,
    pub theme: Arc<Theme>,
    pub root: Option<Arc<Widget>>,
    pub backend: Arc<RefCell<Backend>>,
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

    pub fn with_backend(mut self, backend: Arc<RefCell<Backend>>) -> Self {
        self.backend = backend;
        self
    }

    pub fn build(self) {
        self.backend.borrow_mut().bounds(&self.bounds);
        let mut tree_manager = TreeManager::new(self.backend, self.theme.clone());

        if let Some(root) = self.root {
            tree_manager.root(root.clone());
        }

        let theme = self.theme.clone();
        self.application.windows.push(Window {
            tree_manager,
            bounds: self.bounds,
            title: self.title,
            theme,
            running: true,
        })
    }
}
