use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use orbclient::Window as OrbWindow;
use orbfont::Font;

use self::backend::{OrbitalBackend, OrbitalBackendRunner};
use backend::{Backend, FontMeasure};
use structs::Rect;
use theme::{Theme, ROBOTO_REGULAR_FONT};

mod backend;
mod renderer;

pub fn target_backend(
    title: &str,
    bounds: Rect,
    theme: Theme,
) -> (Box<OrbitalBackendRunner>, Rc<RefCell<Backend>>) {
    let backend = Rc::new(RefCell::new(OrbitalBackend::new(
        theme,
        OrbWindow::new_flags(bounds.x, bounds.y, bounds.width, bounds.height, title, &[]).unwrap(),
    )));

    let backend_runner = Box::new(OrbitalBackendRunner {
        backend: backend.clone(),
        world: None,
    });

    (backend_runner, backend)
}

pub struct OrbFontMeasure {
    font: Option<Font>
}

impl FontMeasure for OrbFontMeasure {
    fn measure(&self, text: &str, font_size: u32) -> (u32, u32) {
        if let Some(font) = &self.font {
            let text = font.render(text, font_size as f32);
            return (text.width(), text.height())
        } 

        (text.chars().count() as u32 * 8 + 2, 18)
    }
}

lazy_static! {
    pub static ref FONT_MEASURE: Arc<OrbFontMeasure> = {
        let font = {
            if let Ok(font) = Font::from_data(ROBOTO_REGULAR_FONT.to_vec().into_boxed_slice()) {
                Some(font)
            } else {
                None
            }
        };

        Arc::new(OrbFontMeasure {
            font
        })
    };
}
