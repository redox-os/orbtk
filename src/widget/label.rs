use backend::Backend;
use dces::ComponentBox;
use structs::Rect;
use theme::Selector;
use {Drawable, Widget};

pub struct Label {
    pub selector: ComponentBox,
}

impl Label {
    pub fn new(selector: Selector) -> Self {
        Label {
            selector: ComponentBox::new(selector),
        }
    }
}

impl Widget for Label {
    fn components(&self) -> Vec<ComponentBox> {
        vec![
            ComponentBox::new(String::from("Label")),
            ComponentBox::new(Selector::new(Some("button"))),
            ComponentBox::new(Drawable::new(Box::new(
                |_bounds: &Rect, selector: &Selector, renderer: &mut Box<Backend>| {
                    renderer.render_text("text", &Rect::new(5, 5, 60, 40), selector);
                },
            ))),
        ]
    }
}
