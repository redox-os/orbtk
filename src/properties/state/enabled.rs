/// The struct `Enabled` is to enable / disable a widget. If `Enabled` is set to `false` the widget could have a different look
/// and its event handler will not be called. All children inherit the enabled state of its parent.
#[derive(Clone, Default, PartialEq)]
pub struct Enabled(pub bool);

property!(Enabled, EnabledProperty, enabled, shared_enabled);

impl From<bool> for Enabled {
    fn from(t: bool) -> Self {
        Enabled(t)
    }
}
