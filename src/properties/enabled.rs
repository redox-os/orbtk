/// The struct `Enabled` is to enable / disable a widget. If `Enabled` is set to `false` the widget could have a different look
/// and its event handler will not be called. All children inherit the enabled state of its parent.
pub struct Enabled(pub bool);