use crate::prelude::*;

/// The `PopupStates` handles the open and close behavior the the `Popup`.
#[derive(Default, AsAny)]
pub struct PopupState {}

impl State for PopupState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if *ctx.widget().get::<bool>("open") {
            ctx.widget().set("visibility", Visibility::Visible);
        } else {
            ctx.widget().set("visibility", Visibility::Collapsed);
            {
                ctx.widget().get_mut::<Rectangle>("bounds").set_width(0.0);
                ctx.widget().get_mut::<Rectangle>("bounds").set_height(0.0);
            }
        }
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if *ctx.widget().get::<Visibility>("visibility") != Visibility::Visible {
            return;
        }

        if let Some(target) = ctx.widget().try_clone::<u32>("target") {
            let target_position: Point = ctx.get_widget(target.into()).clone("position");
            let target_bounds: Rectangle = ctx.get_widget(target.into()).clone("bounds");

            ctx.widget()
                .get_mut::<Rectangle>("bounds")
                .set_x(target_position.x + target_bounds.x());
            ctx.widget()
                .get_mut::<Rectangle>("bounds")
                .set_y(1.0 + target_position.y + target_bounds.y() + target_bounds.height());
        }
    }
}

widget!(
    /// The `Popup` is used to display content that floats over the main content.
    Popup<PopupState> : MouseHandler {
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

        /// Sets or shares the target id to place the popup.
        target: u32,

        /// Sets or shares the value if the popup is open and visible.
        open: bool
    }
);

impl Template for Popup {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Popup")
            .element("popup")
            .open(false)
            .padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .on_mouse_down(|_, _| true)
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PopupLayout::new())
    }
}
