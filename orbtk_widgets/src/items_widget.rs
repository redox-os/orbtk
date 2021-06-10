use crate::{api::prelude::*, proc_macros::*, themes::theme_orbtk::*, Stack};

#[derive(Default, AsAny)]
pub struct ItemsWidgetState {
    builder: WidgetBuildContext,
    count: usize,
}

impl ItemsWidgetState {
    fn generate_items(&mut self, ctx: &mut Context) {
        let count: usize = ctx.widget().clone_or_default("count");
        let request_update: bool = *ctx.widget().get("request_update");

        if count != self.count || request_update {
            if let Some(builder) = &self.builder {
                if let Some(items_panel) = ctx.entity_of_child("items_panel") {
                    ctx.clear_children_of(items_panel);

                    for i in 0..count {
                        let bctx = &mut ctx.build_context();

                        let child = builder(bctx, i);
                        bctx.append_child(items_panel, child);
                    }
                }
            }

            self.count = count;
            ctx.widget().set("request_update", false);
        }
    }
}

impl State for ItemsWidgetState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.generate_items(ctx);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.generate_items(ctx);
    }
}

widget!(
    /// The `ItemsWidget` is a simple no interactive items drawer widget.
    ///
    /// **style:** `items-widget`
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

        /// Use this flag to force the redrawing of the items.
        request_update: bool
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
            .style("items_widget")
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_width(1.0)
            .border_brush(colors::BOMBAY_COLOR)
            .padding(2.0)
            .orientation("vertical")
            .child(Stack::new().id("items_panel").orientation(id).build(ctx))
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        PaddingLayout::new().into()
    }
}
