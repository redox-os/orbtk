use layout_object::{LayoutObject, TextSizeLayoutObject};
use render_object::{RenderObject, TextRenderObject};
use theme::Selector;
use widget::{Property, PropertyResult, Template, Widget};
use widget::PbTemplate;

/// The `Label` struct represents a string used for text drawing.
#[derive(Clone)]
pub struct Label(pub String);

/// The `TextBlock` widget is used to draw text.
pub struct TextBlock {
    pub label: Property<Label>,
    pub selector: Property<Selector>,
}

impl Default for TextBlock {
    fn default() -> TextBlock {
        TextBlock {
            label: Property::new(Label(String::from("TextBlock"))),
            selector: Property::new(Selector::new(Some(String::from("textblock")))),
        }
    }
}

impl Widget for TextBlock {
    fn template(&self) -> Template {
        print!("TextBlock -> ");
        Template::Empty
    }
    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.selector.build(), self.label.build()]
    }

    fn render_object(&self) -> Option<Box<RenderObject>> {
        Some(Box::new(TextRenderObject))
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(TextSizeLayoutObject)
    }
}
