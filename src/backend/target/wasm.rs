use std::rc::Rc;
use std::cell::RefCell;

use {Rect, WasmBackend, Backend, Theme, WasmBackendRunner, CanvasRenderer};

pub fn target_backend(_title: &str, _bounds: Rect, theme: Theme) -> (Box<WasmBackendRunner>, Rc<RefCell<Backend>>) {
    let backend = Rc::new(RefCell::new(WasmBackend::new(theme, CanvasRenderer::new())));

    let backend_runner = Box::new(WasmBackendRunner { backend: backend.clone(), world: None });

    (backend_runner, backend)
}