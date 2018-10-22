use orbclient::{Renderer as OrbRenderer, Window as OrbWindow};

use backend::Renderer;
use structs::Rect;
use theme::{Selector, Theme};

impl Renderer for OrbWindow {
    fn render(&mut self, theme: &Theme) {
        // render window background
        let col = theme.color("background", &"window".into());
        let blub = col.data;
        let mut _color = format!("#{:x}", blub);
        _color.remove(0);
        _color.remove(0);
        self.set(theme.color("background", &"window".into()));
    }

    fn render_rectangle(
        &mut self,
        theme: &Theme,
        bounds: &Rect,
        selector: &Selector,
        _boundery: (u32, u32),
        offset: (i32, i32),
    ) {
        let b_r = theme.uint("border-radius", selector);

        let fill = theme.color("background", selector);

        self.rounded_rect(
            bounds.x + offset.0,
            bounds.y + offset.1,
            bounds.width,
            bounds.height,
            b_r,
            true,
            fill,
        );

        if theme.uint("border-width", selector) > 0 {
            let border_color = theme.color("border-color", selector);

            self.rounded_rect(
                bounds.x + offset.0,
                bounds.y + offset.1,
                bounds.width,
                bounds.height,
                b_r,
                false,
                border_color,
            );
        }
    }

    fn render_text(
        &mut self,
        theme: &Theme,
        text: &str,
        bounds: &Rect,
        selector: &Selector,
        boundery: (u32, u32),
        offset: (i32, i32),
    ) {
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
                if current_rect.x + 8 <= rect.x + boundery.0 as i32
                    && current_rect.y + 16 <= rect.y + boundery.1 as i32
                    && current_rect.x >= 0
                    && current_rect.y >= 0
                {
                    self.char(
                        current_rect.x + offset.0,
                        current_rect.y + offset.1,
                        c,
                        theme.color("color", selector),
                    );
                }
                current_rect.x += 8;
            }
        }
    }
}
