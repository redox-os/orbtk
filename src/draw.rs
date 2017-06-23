use orbclient::Renderer;

use theme::{Theme, Selector};
use rect::Rect;

pub fn draw_box(renderer: &mut Renderer, rect: Rect, theme: &Theme, selector: &Selector) {
    let b_r = theme.uint("border-radius", selector);

    let fill = theme.color("background", selector);

    renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, true, fill);

    if theme.uint("border-thickness", selector) > 0 {
        let border_color = theme.color("border-color", selector);

        renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, false, border_color);
    }
}
