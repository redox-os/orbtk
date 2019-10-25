use super::behaviors::{MouseBehavior, SelectionBehavior};
use crate::prelude::*;

widget!(
    /// The `ToggleButton` widget can be clicked by user and could switch between selected / not selected. 
    /// It's used to perform an action.
    /// 
    /// **CSS element:** `toggle-button`
    ToggleButton: MouseHandler {
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

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or shares the text property.
        text: String16,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the icon property.
        icon: String,

        /// Sets or shares the icon brush property.
        icon_brush: Brush,

        /// Sets or share the icon font size property.
        icon_size: f64,

        /// Sets or shares the icon font property.
        icon_font: String,

        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the pressed property. 
        pressed: bool,

        /// Sets or shares the selected property. 
        selected: bool
    }
);

impl Template for ToggleButton {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("ToggleButton")
            .selector("toggle-button")
            .selected(false)
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
                        SelectionBehavior::create()
                            .selected(id)
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
                    .build(context),
            )
    }
}
