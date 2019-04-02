use dces::prelude::Entity;

use crate::{
    event::ClickHandler,
    properties::*,
    styling::{colors, fonts},
    widgets::{Container, FontIconBlock, Stack, Template, TextBlock},
};

widget!(
    /// The `ToggleButton` widget can be clicked by user and could switch between selected / not selected. 
    /// It's used to perform an action.
    /// 
    /// * CSS element: `toggle-button`
    ToggleButton: ClickHandler {
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

impl Template for ToggleButton {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("ToggleButton")
            .selector("toggle-button")
            .selected(false)
            .height(32.0)
            .min_width(80.0)
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_thickness(0.0)
            .border_brush("transparent")
            .padding((8.0, 0.0, 8.0, 0.0))
            .foreground(colors::LINK_WATER_COLOR)
            .text("")
            .font_size(fonts::FONT_SIZE_12)
            .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
            .icon("")
            .icon_font(fonts::font_into_box(fonts::MATERIAL_ICONS_REGULAR_FONT))
            .icon_size(fonts::ICON_FONT_SIZE_12)
            .icon_brush(colors::LINK_WATER_COLOR)
            .pressed(false)
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_thickness(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Stack::create()
                            .orientation("Horizontal")
                            .vertical_alignment("Center")
                            .horizontal_alignment("Center")
                            .child(
                                FontIconBlock::create()
                                    .margin((0.0, 0.0, 2.0, 0.0))
                                    .icon(id)
                                    .brush(id)
                                    .icon_size(id)
                                    .font(id)
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
    }
}
