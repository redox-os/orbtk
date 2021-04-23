use orbtk_core::{components::*, widget::*};

#[derive(Default, Debug)]
pub struct TextBlock {
    font: FontComponent,
    text: TextComponent,
}

impl TextBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        Self {
            text: TextComponent::new(text),
            ..Default::default()
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text.text = text.into();
        self
    }

    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.font.family = family.into();
        self
    }

    pub fn font_size(mut self, font_size: u32) -> Self {
        self.font.size = font_size;
        self
    }
}

impl Widget for TextBlock {
    fn build(self, btx: &mut BuildContext) {
        todo!()
    }
}
