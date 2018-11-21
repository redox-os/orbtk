use event::EventBox;
use widget::WidgetContainer;

pub trait EventHandler {
    fn handle_event(&self, event: &EventBox, widget: &mut WidgetContainer) -> bool;
}
