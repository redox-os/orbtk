use crate::prelude::*;

/// State to handle the position of switch toggle.
#[derive(Default)]
pub struct ProgressBarState;

impl State for ProgressBarState {
    fn update_post_layout(&self, context: &mut Context<'_>) {
        let mut value = context.widget().get::<Value>().0;

        //check for lower boundary
        if value < 0.0 {
            value = 0.0;
        }

        //check for upper boundary
        if value > 1.0 {
            value = 1.0;
        }
        
        let width = context.widget().get::<Bounds>().width();
        let frame = context.child_by_id("ProgressBar").unwrap();
        let max = width - 2.0 * frame.get::<Padding>().left();

        let mut bar = context.child_by_id("ProgressBarIndicator").unwrap();
        if let Some(bounds) = bar.try_get_mut::<Bounds>() {
                bounds.set_width(max * value);
        }
    }
}

widget!(
    ///
    /// **CSS element:** `switch`
    ProgressBar<ProgressBarState> {

        /// Sets or shares the value property.progressbar
        value: Value,
                /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for ProgressBar {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("ProgressBar")
            .value(0.0)
            .child(
                Container::create()
                    .selector(SelectorValue::from("progressbar").id("ProgressBar"))
                    .padding(4.0)
                    .border_radius(2.0)
                    .child(
                        Container::create()
                            .selector(SelectorValue::from("progressbar-indicator").id("ProgressBarIndicator"))
                            .height(8.0)
                            .vertical_alignment("Center")
                            .horizontal_alignment("Start")
                            .border_radius(1.0)
                            .build(context),
                    )
                    .build(context),
            )
    }
}
