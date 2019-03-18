// use std::rc::Rc;

// use crate::{
//     layout::TextSelectionLayout,
//     properties::{
//         BackgroundProperty, FocusedProperty, OffsetProperty, TextProperty, TextSelection,
//         TextSelectionProperty, FontProperty, FontSizeProperty
//     },
//     render_object::RectangleRenderObject,
//     widget::{
//         add_selector_to_widget, remove_selector_from_widget, Context, State, Template, Widget,
//     },
//     styling::fonts
// };

// // Default state of the `Cursor` widget.
// struct CursorState;

// impl State for CursorState {
//     fn update(&self, context: &mut Context<'_>) {
//         let mut selection_length = 0;
//         let mut widget = context.widget();

//         if let Ok(selection) = widget.borrow_property::<TextSelection>() {
//             selection_length = selection.length;
//         }

//         if selection_length > 0 {
//             add_selector_to_widget("expanded", &mut widget);
//         } else {
//             remove_selector_from_widget("expanded", &mut widget)
//         }
//     }
// }

// widget!(
//     /// The `Cursor` represents a text cursor used to mark text.
//     Cursor
//     (
//         BackgroundProperty,
//         TextProperty,
//         TextSelectionProperty,
//         FontProperty,
//         FontSizeProperty,
//         OffsetProperty,
//         FocusedProperty
//     )
// );

// impl Widget for Cursor {
//     fn create() -> Self {
//         Cursor::new()
//             .width(1.0)
//             .text("")
//             .selector("cursor")
//             .offset(0.0)
//             .background("transparent")
//             .font_size(fonts::FONT_SIZE_12)
//             .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
//             .text_selection(TextSelection::default())
//             .render_object(RectangleRenderObject)
//             .layout(TextSelectionLayout::new())
//             .state(Rc::new(CursorState))
//             .debug_name("Cursor")
//     }
// }