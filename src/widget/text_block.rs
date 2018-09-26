use theme::Selector;
use {
    ComponentBox, LayoutObject, Property, RenderObject, TextRenderObject, TextSizeLayoutObject,
    Widget,
};

pub struct Label(pub String);

pub struct TextBlock {
    pub label: String,
    pub class: String,
}

impl Default for TextBlock {
    fn default() -> TextBlock {
        TextBlock {
            label: String::from("TextBlock"),
            class: String::from("textblock"),
        }
    }
}

impl Widget for TextBlock {
    fn properties(&self) -> Vec<Property> {
        vec![
            ComponentBox::new(Label(self.label.clone())),
            ComponentBox::new(Selector::new(Some(self.class.clone()))),
        ]
    }

    fn render_object(&self) -> Option<Box<RenderObject>> {
        Some(Box::new(TextRenderObject))
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(TextSizeLayoutObject)
    }
}
