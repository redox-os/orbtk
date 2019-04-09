use orbclient::{Window as OrbWindow, EventOption, Renderer, WindowFlag};

use crate::events::*;

/// Window implementation for OrbClient.
pub struct Window {
    inner: OrbWindow,
}

impl super::Window for Window {
    fn set_title(&mut self, title: impl Into<String>) {
        self.inner.set_title(&title.into());
    }

    fn title(&self) -> String {
        self.inner.title()
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.inner.set_pos(x as i32, y as i32);
    }

    fn position(&self) -> (f64, f64) {
        (self.inner.x() as f64, self.inner.y() as f64)
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.inner.set_size(width as u32, height as u32);
    }

    fn size(&self) -> (f64, f64) {
        (self.inner.width() as f64, self.inner.height() as f64)
    }

    fn sync(&mut self) {
        self.inner.sync();
    }

    fn events(&mut self) -> Vec<Event> {
        let mut events: Vec<Event> = vec![];

        for event in self.inner.events() {
            match event.to_option() {
                // EventOption::Quit(_) => events.push(Event::WindowEvent::Close),
                _ => {}
            }
        }

        events
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.inner.sync();
    }
}

impl From<super::WindowBuilder> for Window {
    fn from(builder: super::WindowBuilder) -> Self {
        let mut flags = vec![];
        if builder.resizable {
            flags.push(WindowFlag::Resizable);
        }

        Window {
            inner: OrbWindow::new_flags(
                builder.position.0 as i32,
                builder.position.1 as i32,
                builder.size.0 as u32,
                builder.size.1 as u32,
                &builder.title,
                &flags,
            ).unwrap()
        }
    }
}
