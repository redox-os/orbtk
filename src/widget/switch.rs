// use std::rc::Rc;

// use crate::{
//     enums::Alignment,
//     properties::{
//         HorizontalAlignment, PaddingProperty, PressedProperty, Selected, SelectedProperty,
//     },
//     theme::Selector,
//     widget::{Container, Context, Grid, Property, State, Template, Widget},
// };

// // State to handle the position of switch toggle.
// struct SwitchState;

// impl State for SwitchState {
//     fn update(&self, context: &mut Context<'_>) {
//         let mut selected = false;
//         if let Ok(sel) = context.widget().borrow_property::<Selected>() {
//             selected = sel.0;
//         }

//         let mut switch_toggle = context.child_by_id("SwitchSwitchToggle").unwrap();

//         if let Ok(horizontal_alignment) = switch_toggle.borrow_mut_property::<HorizontalAlignment>()
//         {
//             if selected {
//                 *horizontal_alignment = HorizontalAlignment(Alignment::End);
//             } else {
//                 *horizontal_alignment = HorizontalAlignment(Alignment::Start);
//             }
//         }
//     }
// }

// widget!(
//     // The `Switch` widget can be switch between `on` and `off`.
//     Switch
//     ( PressedProperty, SelectedProperty )
// );

// impl Widget for Switch {
//     fn create() -> Self {
//         let selector: Property = Selector::from("switch").into();
//         let selected: Property = Selected::from(false).into();

//         Switch::new()
//             .width(56.0)
//             .height(32.0)
//             .state(Rc::new(SwitchState))
//             .debug_name("Switch")
//             .child(
//                 Container::create()
//                     .padding(4.0)
//                     .shared_selector(selector.share())
//                     .child(
//                         Grid::create().child(
//                             Container::create()
//                                 .size(24.0, 24.0)
//                                 .vertical_alignment("Center")
//                                 .horizontal_alignment("Start")
//                                 .attach(selected.share())
//                                 .selector(Selector::from("switchtoggle").id("SwitchSwitchToggle")),
//                         ),
//                     ),
//             )
//             .shared_selector(selector)
//             .shared_selected(selected)
//     }
// }

use dces::prelude::Entity;

use crate::{
    properties::*,
    styling::colors,
    theme::{Selector, SelectorValue},
    widget::{Container, Grid, Template},
};

widget!(
    /// The `Switch` widget can be switch between `on` and `off`.
    ///
    /// * CSS element: `switch`
    Switch {
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

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the pressed property.
        pressed: Pressed,

        /// Sets or shares the selected property.
        selected: Selected
    }
);

impl Template for Switch {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("Switch")
            .selector("switch")
            .pressed(false)
            .selected(false)
            .width(56.0)
            .height(32.0)
            .border_brush(colors::BOMBAY_COLOR)
            .background(colors::SLATE_GRAY_COLOR)
            .border_radius(2.0)
            .border_thickness(1.0)
            .padding(4.0)
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_thickness(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Grid::create()
                            .child(Container::create().size(24.0, 24.0).build(context))
                            .selector(
                                SelectorValue::new()
                                    .with("switch-toggle")
                                    .id("SwitchSwitchToggle"),
                            )
                            .vertical_alignment("Center")
                            .horizontal_alignment("Start")
                            // .attach(id)
                            .build(context),
                    )
                    .build(context),
            )
    }
}
