use orbclient::{Renderer as OrbRenderer, Window as OrbWindow};

use backend::Renderer;
use structs::{Point, Rect};
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
        parent_bounds: &Rect,
        selector: &Selector,
        offset: &Point,
        global_position: &Point,
    ) {
        let is_debug = selector.element.as_ref().map_or(false, |e| e == "debugborder");
        let b_r = theme.uint("border-radius", selector);

        let fill = theme.color("background", selector);

        let x = (bounds.x + global_position.x + offset.x).max(parent_bounds.x);
        let y = (bounds.y + global_position.y + offset.y).max(parent_bounds.y);
        let width = (bounds.width as i32 + offset.x).min(parent_bounds.width as i32) as u32;
        let height = (bounds.height as i32 + offset.y).min(parent_bounds.height as i32) as u32;

        let border_width = theme.uint("border-width", selector);
        let border_half = (border_width / 2) as i32;

        if border_width > 0 {
            let border_color = theme.color("border-color", selector);
            self.rounded_rect(x, y, width + border_width, height + border_width, b_r, !is_debug, border_color);
        }

        self.rounded_rect(x + border_half, y + border_half, width, height, b_r, !is_debug, fill);
    }

    fn render_text(
        &mut self,
        theme: &Theme,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        selector: &Selector,
        offset: &Point,
        global_position: &Point,
    ) {
        // if let Some(font) = &self.font {
        //     let line = font.render(text, 64.0);
        //     line.draw(&mut self.inner, 20, 20, Color::rgb(0, 0, 0));
        // } else {
        let rect = Rect::new(bounds.x, bounds.y, bounds.width, bounds.height);
        let mut current_rect = Rect::new(
            bounds.x + offset.x,
            bounds.y + offset.y,
            bounds.width,
            bounds.height,
        );
        let x = rect.x;

        for c in text.chars() {
            if c == '\n' {
                current_rect.x = x;
                current_rect.y += 16;
            } else {
                if current_rect.x + 8 >= parent_bounds.x
                    && current_rect.y + 16 >= parent_bounds.y
                    && current_rect.x + 8 < parent_bounds.x + parent_bounds.width as i32
                    && current_rect.y < parent_bounds.y + parent_bounds.height as i32
                {
                    self.char(
                        current_rect.x + global_position.x,
                        current_rect.y + global_position.y,
                        c,
                        theme.color("color", selector),
                    );
                }
                current_rect.x += 8;
            }
        }
    }
}
