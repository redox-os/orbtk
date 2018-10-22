use std::cell::RefCell;
use std::rc::Rc;

use self::backend::{WasmBackend, WasmBackendRunner};
use self::renderer::CanvasRenderer;

use backend::Backend;
use structs::Rect;
use theme::Theme;

mod backend;
mod renderer;

pub fn target_backend(
    _title: &str,
    _bounds: Rect,
    theme: Theme,
) -> (Box<WasmBackendRunner>, Rc<RefCell<Backend>>) {
    let backend = Rc::new(RefCell::new(WasmBackend::new(theme, CanvasRenderer::new())));

    let backend_runner = Box::new(WasmBackendRunner {
        backend: backend.clone(),
        world: None,
    });

    (backend_runner, backend)
}
