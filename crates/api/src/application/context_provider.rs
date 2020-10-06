use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
    sync::mpsc,
};

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
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handler_map: Rc<RefCell<EventHandlerMap>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Box<dyn State>>>>,
    pub event_adapter: EventAdapter,
    pub message_adapter: MessageAdapter,
    pub mouse_position: Rc<Cell<Point>>,
    pub window_sender: mpsc::Sender<WindowRequest>,
    pub shell_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
    pub application_name: String,
    pub first_run: Rc<Cell<bool>>,
    pub raw_window_handle: Option<raw_window_handle::RawWindowHandle>,
    // todo thread safe
    pub localization: Option<Rc<RefCell<Box<dyn Localization>>>>,
}

impl ContextProvider {
    /// Creates a new context provider.
    pub fn new(
        window_sender: mpsc::Sender<WindowRequest>,
        shell_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
        application_name: impl Into<String>,
        localization: Option<Rc<RefCell<Box<dyn Localization>>>>,
    ) -> Self {
        ContextProvider {
            render_objects: Rc::new(RefCell::new(BTreeMap::new())),
            layouts: Rc::new(RefCell::new(BTreeMap::new())),
            handler_map: Rc::new(RefCell::new(EventHandlerMap::new())),
            states: Rc::new(RefCell::new(BTreeMap::new())),
            event_adapter: EventAdapter::new(window_sender.clone()),
            message_adapter: MessageAdapter::new(window_sender.clone()),
            mouse_position: Rc::new(Cell::new(Point::new(0.0, 0.0))),
            window_sender,
            shell_sender,
            application_name: application_name.into(),
            first_run: Rc::new(Cell::new(true)),
            raw_window_handle: None,
            localization,
        }
    }
}
