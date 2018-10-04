use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;

use {Application, Backend, EventManager, Rect, RenderContainer, Theme, TreeManager, Widget};

pub struct Window {
    bounds: Rect,
    _title: String,
    theme: Arc<Theme>,
    _running: bool,
    backend: Box<Backend>,
    tree_manager: Option<JoinHandle<()>>,
    root: Option<Arc<Widget>>,
}

impl Window {
    pub fn run(&mut self) {
        let (event_sender, event_receiver): (
            mpsc::Sender<EventManager>,
            mpsc::Receiver<EventManager>,
        ) = mpsc::channel();

        let (render_sender, render_receiver): (
            mpsc::Sender<Vec<RenderContainer>>,
            mpsc::Receiver<Vec<RenderContainer>>,
        ) = mpsc::channel();

        self.backend.bounds(&self.bounds);
        self.backend.event_sender(event_sender.clone());
        self.backend.render_receiver(render_receiver);

        let tree_manager = TreeManager::new(
            self.theme.clone(),
            self.root.clone(),
            Some(event_receiver),
            render_sender.clone(),
            (self.bounds.width, self.bounds.height),
        );

        self.tree_manager = Some(tree_manager);

        self.backend.run();
    }
}

pub struct WindowBuilder<'a> {
    pub application: &'a mut Application,
    pub bounds: Rect,
    pub title: String,
    pub theme: Arc<Theme>,
    pub root: Option<Arc<Widget>>,
    pub backend: Box<Backend>,
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

    pub fn with_backend(mut self, backend: Box<Backend>) -> Self {
        self.backend = backend;
        self
    }

    pub fn build(self) {
        let theme = self.theme.clone();
        self.application.windows.push(Window {
            bounds: self.bounds,
            root: self.root,
            _title: self.title,
            theme: theme.clone(),
            _running: true,
            backend: self.backend,
            tree_manager: None,
        })
    }
}
