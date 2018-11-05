use event::EventBox;
use widget::WidgetContainer;

pub trait EventHandler {
    fn handles_event(&self, event: &EventBox, widget: &WidgetContainer) -> bool;

    fn handle_event(&self, event: &EventBox, widget: &mut WidgetContainer) -> bool;
}
