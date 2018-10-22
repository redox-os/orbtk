use event::EventBox;
use widget::{WidgetContainer, PropertyResult};

pub trait State {
    fn handles_event(
        &self,
        _event: &EventBox,
        _widget: &WidgetContainer,
    ) -> bool {
        false
    }

    fn update(
        &self,
        _event: &EventBox,
        _widget: &mut WidgetContainer,
    ) -> bool {
        false
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![]
    }
}
