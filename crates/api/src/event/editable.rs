use super::{Event, EventBox, EventHandler};
use crate::prelude::*;

use std::rc::Rc;

crate::trigger_event!(
    ChangedEvent,
    ChangedEventHandler,
    ChangedHandler,
    on_changed
);
crate::trigger_event!(
    ActivateEvent,
    ActivateEventHandler,
    ActivateHandler,
    on_activate
);
