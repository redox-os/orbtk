use super::Event;
use orbtk_proc_macros::Event;

#[derive(Event)]
pub enum SystemEvent {
    Quit,
}
