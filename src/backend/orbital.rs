use orbfont;
use std::cell::Cell;
use std::sync::Arc;

use orbclient::{Color, Window as OrbWindow};
use orbclient::{Mode, Renderer as OrbRenderer};

use {Backend, Rect, Renderer, RenderContext, Selector, Theme};

pub struct OrbitalBackend {
    inner: OrbWindow,
    font: Option<orbfont::Font>,
    theme: Arc<Theme>,
}

impl Renderer for OrbWindow {
    fn render(&mut self, theme: &Arc<Theme>) {
        // render window background
        self            .set(theme.color("background", &"window".into()));

        // 'events: loop {
        //     for event in self.inner.events() {
        //         match event.to_option() {
        //             EventOption::Quit(_quit_event) => break 'events,
        //             EventOption::Mouse(evt) => println!(
        //                 "At position {:?} pixel color is : {:?}",
        //                 (evt.x, evt.y),
        //                 self.inner.getpixel(evt.x, evt.y)
        //             ),
        //             event_option => println!("{:?}", event_option),
        //         }
        //     }
        // }
    }

    fn render_rectangle(&mut self, theme: &Arc<Theme>, bounds: &Rect, selector: &Selector) {
        let b_r = theme.uint("border-radius", selector);

        let fill = theme.color("background", selector);

        self.rounded_rect(
            bounds.x,
            bounds.y,
            bounds.width,
            bounds.height,
            b_r,
            true,
            fill,
        );

        if theme.uint("border-width", selector) > 0 {
            let border_color = theme.color("border-color", selector);

            self.rounded_rect(
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                b_r,
                false,
                border_color,
            );
        }
    }

    fn render_text(&mut self, theme: &Arc<Theme>, text: &str, bounds: &Rect, selector: &Selector) {
        // if let Some(font) = &self.font {
        //     let line = font.render(text, 64.0);
        //     line.draw(&mut self.inner, 20, 20, Color::rgb(0, 0, 0));
        // } else {
            let rect = Rect::new(bounds.x, bounds.y, bounds.width, bounds.height);
            let mut current_rect = Rect::new(bounds.x, bounds.y, bounds.width, bounds.height);
            let x = rect.x;

            for c in text.chars() {
                if c == '\n' {
                    current_rect.x = x;
                    current_rect.y += 16;
                } else {
                    if current_rect.x + 8 <= rect.x + rect.width as i32
                        && current_rect.y + 16 <= rect.y + rect.height as i32
                    {
                        self.char(
                            current_rect.x,
                            current_rect.y,
                            c,
                            theme.color("color", selector),
                        );
                    }
                    current_rect.x += 8;
                }
            }
        // }
    }

}


impl OrbitalBackend {
    pub fn new(inner: OrbWindow, font: Option<orbfont::Font>, theme: Arc<Theme>) -> OrbitalBackend {
        OrbitalBackend {
            inner: inner,
            font: font,
            theme,
        }
    }
}

impl OrbRenderer for OrbitalBackend {
    fn width(&self) -> u32 {
        self.inner.width()
    }

    fn height(&self) -> u32 {
        self.inner.height()
    }

    fn data(&self) -> &[Color] {
        self.inner.data()
    }

    fn data_mut(&mut self) -> &mut [Color] {
        self.inner.data_mut()
    }

    fn sync(&mut self) -> bool {
        self.inner.sync()
    }

    fn mode(&self) -> &Cell<Mode> {
        &self.inner.mode()
    }

    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        if let Some(ref font) = self.font {
            let mut buf = [0; 4];
            font.render(&c.encode_utf8(&mut buf), 16.0)
                .draw(&mut self.inner, x, y, color)
        } else {
            self.inner.char(x, y, c, color);
        }
    }
}

impl Drop for OrbitalBackend {
    fn drop(&mut self) {
        self.inner.sync();
    }
}

impl Backend for OrbitalBackend {
    

    fn update(&mut self) {
        self.inner.sync();
        for _event in self.inner.events() {}
    }

    
    fn bounds(&mut self, bounds: &Rect) {
        self.inner.set_pos(bounds.x, bounds.y);
        self.inner.set_size(bounds.width, bounds.height);
    }

    fn render_context(&mut self) -> RenderContext {
        RenderContext {
            renderer: &mut self.inner,
            theme:  self.theme.clone()
        }
    }
}
