use super::Point;
use super::Event;

pub trait Click {
    fn emit_click(&self, point: Point);
    fn on_click<T: Fn(&Self, Point) + 'static>(self, func: T) -> Self;
}

pub trait Enter {
    fn emit_enter(&self);
    fn on_enter<T: Fn(&Self) + 'static>(self, func: T) -> Self;
}

pub trait EventFilter {
    fn handle_event(&self, event: Event, focused: &mut bool, redraw: &mut bool) -> Option<Event>;
    fn event_filter<T: Fn(&Self, Event, &mut bool, &mut bool) -> Option<Event> + 'static>(self, func: T) -> Self;
}
