use crate::prelude::*;

/// The `CheckBoxState` handles the `CheckBox` widget.
#[derive(Default)]
pub struct CheckBoxState {}

impl PressedState for CheckBoxState {}
impl SelectedState for CheckBoxState {}

impl Into<Rc<dyn State>> for CheckBoxState {
    fn into(self) -> Rc<dyn State> {
        Rc::new(self)
    }
}

impl State for CheckBoxState {
    fn update(&self, context: &mut Context<'_>) {
        self.update_pressed(&mut context.widget());
        self.update_selected(&mut context.widget());
    }
}

widget!(
    /// The `CheckBox` widget can be switch its selected state. It contains a selection box and a text.
    /// 
    /// **CSS element:** `check-box`
    CheckBox<CheckBoxState>: ClickHandler {
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

        /// Sets or shares the foreground property.
        foreground: Foreground,

        /// Sets or shares the text property.
        text: Text,

        /// Sets or share the font size property.
        font_size: FontSize,

        /// Sets or shares the font property.
        font: Font,

        /// Sets or shares the icon property.
        icon: FontIcon,

        /// Sets or shares the icon brush property.
        icon_brush: IconBrush,

        /// Sets or share the icon font size property.
        icon_size: IconSize,

        /// Sets or shares the icon font property.
        icon_font: IconFont,

        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the pressed property. 
        pressed: Pressed,

        /// Sets or shares the selected property. 
        selected: Selected
    }
);

impl Template for CheckBox {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("CheckBox")
            .selector("check-box")
            .selected(false)
            .height(24.0)
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_thickness(0.0)
            .border_brush("transparent")
            .padding((8.0, 0.0, 8.0, 0.0))
            .foreground(colors::LINK_WATER_COLOR)
            .text("")
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
            .icon(material_font_icons::CHECK_FONT_ICON)
            .icon_font("Material Icons")
            .icon_size(fonts::ICON_FONT_SIZE_12)
            .icon_brush(colors::LINK_WATER_COLOR)
            .pressed(false)
            .child(
                Stack::create()
                    .orientation("Horizontal")
                    .child(
                        Container::create()
                            .size(24.0, 24.0)
                            .background(id)
                            .border_radius(id)
                            .border_thickness(id)
                            .border_brush(id)
                            .padding(id)
                            .child(
                                FontIconBlock::create()
                                    .vertical_alignment("Center")
                                    .horizontal_alignment("Center")
                                    .icon(id)
                                    .brush(id)
                                    .icon_size(id)
                                    .font(id)
                                    .build(context),
                            )
                            .build(context),
                    )
                    .child(
                        TextBlock::create()
                            .vertical_alignment("Center")
                            .margin((8.0, 0.0, 0.0, 0.0))
                            .foreground(id)
                            .text(id)
                            .font_size(id)
                            .font(id)
                            .build(context),
                    )
                    .build(context),
            )
    }
}
