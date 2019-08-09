use crate::prelude::*;

#[derive(Default)]
pub struct ItemsWidgetState {}

impl Into<Rc<dyn State>> for ItemsWidgetState {
    fn into(self) -> Rc<dyn State> {
        Rc::new(self)
    }
}

impl State for ItemsWidgetState {
    fn update(&self, context: &mut Context<'_>) {
        if let Some(items_panel) = context.entity_of_child("items_panel") {
            context.append_child_to(items_panel, Button::create().text("Test"))
        }
    }
}

widget!(
    /// The `ItemsWidget` is a simple no interactive items drawer widget.
    ///
    /// **CSS element:** `items-widget`
    ItemsWidget<ItemsWidgetState> {
        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_thickness: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the padding property.
        padding: Padding,

        /// Sets or shares the orientation property.
        orientation: Orientation,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for ItemsWidget {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("ItemsWidget")
            .selector("items-widget")
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_thickness(1.0)
            .border_brush(colors::BOMBAY_COLOR)
            .padding(2.0)
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_thickness(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Stack::create()
                            .selector(SelectorValue::default().clone().id("items_panel"))
                            .orientation(id)
                            .build(context),
                    )
                    .build(context),
            )
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}
