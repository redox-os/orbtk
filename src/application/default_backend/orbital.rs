use std::sync::Arc;

use {OrbitalBackend, Theme};

pub fn default_backend(theme: Arc<Theme>) -> Box<OrbitalBackend> {
    Box::new(OrbitalBackend::new(theme))
}