use crate::prelude::*;

/// The `PressedHelperState` handles the `PressedHelper` widget.
#[derive(Default)]
pub struct PressedHelperState;

impl State for PressedHelperState {
    fn update(&self, context: &mut Context<'_>) {
         if context.widget().get::<Pressed>().0 {
            add_selector_to_widget("active", &mut context.widget());
        } else {
            remove_selector_from_widget("active", &mut context.widget());
        }
    }
}

widget!(
    /// The `PressedHelper` widget is used to handle internal the pressed behavior of a widget.
    /// 
    /// **CSS element:** `check-box`
    PressedHelper<PressedHelperState> {
        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the pressed property. 
        pressed: Pressed
    }
);

impl Template for PressedHelper {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("PressedHelper")
            .pressed(false)
    }
}
