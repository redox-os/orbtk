use std::sync::Arc;

use {WasmBackend, Theme};

pub fn default_backend(theme: Arc<Theme>) -> Box<WasmBackend> {
    Box::new(WasmBackend::new(theme))
}