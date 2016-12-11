use event::Event;

pub trait EventFilter {
    fn handle_event(&self, event: Event, focused: &mut bool, redraw: &mut bool) -> Option<Event>;
    fn event_filter<T: Fn(&Self, Event, &mut bool, &mut bool) -> Option<Event> + 'static>(&self, func: T) -> &Self;
}
