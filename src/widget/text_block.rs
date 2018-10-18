use theme::Selector;
use {
    LayoutObject, Property, RenderObject, TextRenderObject, TextSizeLayoutObject,
    Widget, Key,
};

pub struct Label(pub String);

pub struct TextBlock {
    pub label: String,
    pub class: String,
    pub key: String,
}

impl Default for TextBlock {
    fn default() -> TextBlock {
        TextBlock {
            label: String::from("TextBlock"),
            class: String::from("textblock"),
            key: String::from("TextBlock"),
        }
    }
}

impl Widget for TextBlock {
    fn properties(&self) -> Vec<Property> {
        vec![
            Property::new(Label(self.label.clone())),
            Property::new(Selector::new(Some(self.class.clone()))),
            Property::new(Key(self.key.clone())),
        ]
    }

    fn render_object(&self) -> Option<Box<RenderObject>> {
        Some(Box::new(TextRenderObject))
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(TextSizeLayoutObject)
    }
}
