use std::cell::Cell;

use crate::prelude::*;

#[derive(Default)]
pub struct ItemsWidgetState {
    builder: RefCell<Option<Box<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>>,
    count: Cell<usize>,
}

impl Into<Rc<dyn State>> for ItemsWidgetState {
    fn into(self) -> Rc<dyn State> {
        Rc::new(self)
    }
}

impl State for ItemsWidgetState {
    fn update(&self, context: &mut Context<'_>) {
        let count = context.widget().clone_or_default::<usize>("count");

        if count != self.count.get() {
            if let Some(builder) = &*self.builder.borrow() {
                if let Some(items_panel) = context.entity_of_child("items_panel") {
                    context.clear_children_of(items_panel);

                    for i in 0..count {
                        let child = {
                            let mut build_context = context.build_context();
                            let child = builder(&mut build_context, i);
                            build_context.append_child(items_panel, child);
                            child
                        };

                        context.get_widget(child).update_properties_by_theme();
                    }
                }
            }

            self.count.set(count);
        }
    }
}

widget!(
    /// The `ItemsWidget` is a simple no interactive items drawer widget.
    ///
    /// **CSS element:** `items-widget`
    ItemsWidget<ItemsWidgetState> {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the orientation property.
        orientation: Orientation,

        /// Sets or shared the count.
        count: usize,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl ItemsWidget {
    pub fn items_builder<F: Fn(&mut BuildContext, usize) -> Entity + 'static>(
        self,
        builder: F,
    ) -> Self {
        *self.clone_state().builder.borrow_mut() = Some(Box::new(builder));
        self
    }
}

impl Template for ItemsWidget {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("ItemsWidget")
            .selector("items-widget")
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_width(1.0)
            .border_brush(colors::BOMBAY_COLOR)
            .padding(2.0)
            .orientation("vertical")
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_width(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Stack::create()
                            .selector(Selector::default().id("items_panel"))
                            .orientation(id)
                            .build(context),
                    )
                    .build(context),
            )
    }
}
