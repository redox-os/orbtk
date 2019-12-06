use crate::prelude::*;

#[derive(Default, AsAny)]
pub struct ItemsWidgetState {
    builder: WidgetBuildContext,
    count: usize,
}

impl State for ItemsWidgetState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let count = ctx.widget().clone_or_default::<usize>("count");

        if count != self.count {
            if let Some(builder) = &self.builder {
                if let Some(items_panel) = ctx.entity_of_child("items_panel") {
                    ctx.clear_children_of(items_panel);

                    for i in 0..count {
                        let bctx = &mut ctx.build_context();

                        let child = {
                            let child = builder(bctx, i);
                            bctx.append_child(items_panel, child);
                            child
                        };

                        ctx.get_widget(child).update_properties_by_theme();
                    }
                }
            }

            self.count = count;
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
        mut self,
        builder: F,
    ) -> Self {
        self.state_mut().builder = Some(Box::new(builder));
        self
    }
}

impl Template for ItemsWidget {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
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
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
