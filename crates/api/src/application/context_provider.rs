use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
    sync::mpsc,
};

use std::sync::{atomic::AtomicBool, Arc, RwLock};

use dces::prelude::*;

use super::WindowAdapter;

use crate::{
    event::*,
    layout::*,
    localization::Localization,
    render_object::*,
    shell::{ShellRequest, WindowRequest},
    utils::Point,
    widget_base::*,
};

/// Temporary solution to share dependencies. Will be refactored soon.
#[derive(Clone)]
pub struct ContextProvider {
    pub render_objects: Arc<RwLock<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Arc<RwLock<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handler_map: Arc<RwLock<EventHandlerMap>>,
    pub states: Arc<RwLock<BTreeMap<Entity, Box<dyn State>>>>,
    pub event_queue: Arc<RwLock<EventQueue>>,
    pub mouse_position: Arc<RwLock<Point>>,
    pub window_sender: mpsc::Sender<WindowRequest>,
    pub shell_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
    pub application_name: String,
    pub first_run: Arc<AtomicBool>,
    pub raw_window_handle: Option<raw_window_handle::RawWindowHandle>,
    // todo thread save
    pub localization: Option<Arc<RwLock<Box<dyn Localization>>>>,
}

impl ContextProvider {
    /// Creates a new context provider.
    pub fn new(
        window_sender: mpsc::Sender<WindowRequest>,
        shell_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
        application_name: impl Into<String>,
        localization: Option<Arc<RwLock<Box<dyn Localization>>>>,
    ) -> Self {
        ContextProvider {
            render_objects: Arc::new(RwLock::new(BTreeMap::new())),
            layouts: Arc::new(RwLock::new(BTreeMap::new())),
            handler_map: Arc::new(RwLock::new(EventHandlerMap::new())),
            states: Arc::new(RwLock::new(BTreeMap::new())),
            event_queue: Arc::new(RwLock::new(EventQueue::new())),
            mouse_position: Arc::new(RwLock::new(Point::new(0.0, 0.0))),
            window_sender,
            shell_sender,
            application_name: application_name.into(),
            first_run: Arc::new(AtomicBool::new(true)),
            raw_window_handle: None,
            localization,
        }
    }
}
