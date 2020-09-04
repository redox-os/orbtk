use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - widgets example")
                .position((100, 100))
                .size(468, 730)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

widget!(MainView {});

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            TabWidget::new()
                .tab("Buttons", ButtonView::new().build(ctx))
                .tab("Text", TextView::new().build(ctx))
                .tab("Items", ItemsView::new().build(ctx))
                .tab("Layouts", LayoutView::new().build(ctx))
                .tab("Image", ImageView::new().build(ctx))
                .build(ctx),
        )
    }
}

widget!(ButtonView {});

impl Template for ButtonView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .width(140)
                .margin(16)
                .spacing(8)
                .child(
                    Button::new()
                        .text("Button")
                        .icon(material_icons_font::MD_CHECK)
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .text("Primary")
                        .style("button_primary")
                        .icon(material_icons_font::MD_360)
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .text("Text only")
                        .style("button_single_content")
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .icon(material_icons_font::MD_CHECK)
                        .style("button_single_content")
                        .build(ctx),
                )
                .child(
                    ToggleButton::new()
                        .text("ToggleButton")
                        .icon(material_icons_font::MD_ALARM_ON)
                        .build(ctx),
                )
                .child(CheckBox::new().text("CheckBox").build(ctx))
                .child(Switch::new().build(ctx))
                .build(ctx),
        )
    }
}

widget!(TextView {});

impl Template for TextView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .width(140)
                .margin(16)
                .spacing(8)
                .child(TextBlock::new().text("Header").style("header").build(ctx))
                .child(TextBlock::new().text("Text").style("body").build(ctx))
                .child(TextBox::new().water_mark("Insert text...").build(ctx))
                .child(
                    PasswordBox::new()
                        .water_mark("Insert password...")
                        .build(ctx),
                )
                .child(NumericBox::new().max(123).step(0.123).val(0.123).build(ctx))
                .build(ctx),
        )
    }
}

widget!(ItemsView {});

impl Template for ItemsView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self
    }
}

widget!(LayoutView {});

impl Template for LayoutView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self
    }
}

widget!(ImageView {});

impl Template for ImageView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            ImageWidget::new()
                .margin(16)
                .image("res/orbtk_space.png")
                .build(ctx),
        )
    }
}
