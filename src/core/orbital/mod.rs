//! This module contains the concrete implementation of the OrbClient based backend.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use orbclient::{Window as OrbWindow, WindowFlag};

use self::backend::{OrbitalBackend, OrbitalBackendRunner};
pub use self::render_context_2d::{FONT_RENDERER};
use crate::core::{Backend, FontMeasure};
use crate::properties::Bounds;
use crate::theme::Theme;

mod backend;
mod render_context_2d;

pub fn target_backend(
    title: &str,
    bounds: Bounds,
    resizable: bool,
    theme: Theme,
) -> (Box<OrbitalBackendRunner>, Rc<RefCell<dyn Backend>>) {
    let flags = {
        if resizable {
            vec![WindowFlag::Resizable]
        } else {
            vec![]
        }
    };

    let backend = Rc::new(RefCell::new(OrbitalBackend::new(
        theme,
        OrbWindow::new_flags(bounds.x, bounds.y, bounds.width, bounds.height, title, &flags).unwrap(),
    )));

    let backend_runner = Box::new(OrbitalBackendRunner {
        backend: backend.clone(),
        world: None,
    });

    (backend_runner, backend)
}

pub struct OrbFontMeasure;

impl FontMeasure for OrbFontMeasure {
    fn measure(&self, text: &str, font: &str, font_size: u32) -> (u32, u32) {
        if let Some(font) = &FONT_RENDERER.fonts.get(font) {
            let text = font.render(text, font_size as f32);
            return (text.width(), text.height());
        }

        (text.chars().count() as u32 * 8 + 2, 18)
    }
}

lazy_static! {
    pub static ref FONT_MEASURE: Arc<OrbFontMeasure> = {
        Arc::new(OrbFontMeasure)
    };
}
