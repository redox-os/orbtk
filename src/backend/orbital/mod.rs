//! This module contains the concrete implementation of the OrbClient based backend.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use orbclient::{Window as OrbWindow, WindowFlag};
use orbfont::Font;

pub use self::backend::{WindowBuilder, OrbitalBackend, ShellRunner};

use crate::backend::*;

mod backend;
mod renderer;

// pub fn target_backend(
//     title: &str,
//     bounds: Bounds,
//     resizable: bool,
// ) -> (Box<ShellRunner>, Rc<RefCell<dyn Backend>>) {
//     let mut flags = vec![];
//     if resizable {
//         flags.push(WindowFlag::Resizable);
//     }

//     let backend = Rc::new(RefCell::new(OrbitalBackend::new(
//         OrbWindow::new_flags(
//             bounds.x() as i32,
//             bounds.y() as i32,
//             bounds.width() as u32,
//             bounds.height() as u32,
//             title,
//             &flags,
//         )
//             .unwrap(),
//     )));

//     let backend_runner = Box::new(ShellRunner {
//         backend: backend.clone(),
//         world: None,
//     });

//     (backend_runner, backend)
// }

pub struct OrbFontMeasure;

impl FontMeasure for OrbFontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32) {
        if font_size == 0 {
            return (0, 0);
        }
        let text = font.render(text, font_size as f32);
        (text.width(), text.height())
    }
}

lazy_static! {
    pub static ref FONT_MEASURE: Arc<OrbFontMeasure> = { Arc::new(OrbFontMeasure) };
}
