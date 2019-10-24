use super::behaviors::MouseBehavior;
use crate::prelude::*;

widget!(
    /// The `Button` widget can be clicked by user. It's used to perform an action.
    /// 
    /// **CSS element:** `button`
    Button: MouseHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_width: BorderThickness,

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
        icon_size: FontSize,

        /// Sets or shares the icon font property.
        icon_font: Font,

        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the pressed property. 
        pressed: Pressed
    }
);

impl Template for Button {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("Button")
            .selector("button")
            .height(32.0)
            .min_width(80.0)
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_width(0.0)
            .border_brush("transparent")
            .padding((8.0, 0.0, 8.0, 0.0))
            .foreground(colors::LINK_WATER_COLOR)
            .text("")
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
            .icon("")
            .icon_font("Material Icons")
            .icon_size(fonts::ICON_FONT_SIZE_12)
            .icon_brush(colors::LINK_WATER_COLOR)
            .pressed(false)
            .child(
                MouseBehavior::create()
                    .pressed(id)
                    .enabled(id)
                    .selector(id)
                    .child(
                        Container::create()
                            .background(id)
                            .border_radius(id)
                            .border_width(id)
                            .border_brush(id)
                            .padding(id)
                            .child(
                                Stack::create()
                                    .orientation("horizontal")
                                    .vertical_alignment("center")
                                    .horizontal_alignment("center")
                                    .child(
                                        FontIconBlock::create()
                                            .margin((0.0, 0.0, 2.0, 0.0))
                                            .icon(id)
                                            .icon_brush(id)
                                            .icon_size(id)
                                            .icon_font(id)
                                            .build(context),
                                    )
                                    .child(
                                        TextBlock::create()
                                            .foreground(id)
                                            .text(id)
                                            .font_size(id)
                                            .font(id)
                                            .build(context),
                                    )
                                    .build(context),
                            )
                            .build(context),
                    )
                    .build(context),
            )
    }
}
