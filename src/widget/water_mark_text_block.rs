use crate::enums::Visibility;
use std::rc::Rc;
use crate::properties::{Label, WaterMark};
use crate::theme::Selector;
use crate::widget::{Context, State, Template, TextBlock, Widget, add_selector_to_widget, remove_selector_from_widget};

/// The `WaterMarkTextBlockState` handles the text processing of the `WaterMarkTextBlock` widget.
#[derive(Default)]
pub struct WaterMarkTextBlockState;

impl Into<Rc<State>> for WaterMarkTextBlockState {
    fn into(self) -> Rc<State> {
        Rc::new(self)
    }
}

impl State for WaterMarkTextBlockState {
    fn update(&self, context: &mut Context) {
        let mut widget = context.widget();

        let mut is_label_empty = false;
        let mut is_water_mark_empty = false;

        if let Ok(label) = widget.borrow_property::<Label>() {
            is_label_empty = label.0.is_empty();
        }

        if is_label_empty {
             add_selector_to_widget("watermark", &mut widget);
        } else {
             remove_selector_from_widget("watermark", &mut widget);
        }

        if let Ok(label) = widget.borrow_property::<WaterMark>() {
            is_water_mark_empty = label.0.is_empty();
        }

        if let Ok(visibility) = widget.borrow_mut_property::<Visibility>() {
            if is_water_mark_empty && !is_label_empty {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
            }
        }
    }
}

/// The `WaterMarkTextBlock` widget is used to display a placeholder watermark if the `Label` is empty.
/// Derives from `TextBlock`.
/// 
/// # Properties
/// 
/// * `Watermark` - String used to display a placeholder text if `Label` string is empty.
/// * `Selector` - CSS selector with  element name `textblock` and class `watermark`, used to request the theme of the WaterMarkTextBlock.
/// 
/// # Others
/// 
/// * `ParentType`- None.
/// * `WaterMarkTextBlockState` - Handles the inner state of the widget.
pub struct WaterMarkTextBlock;

impl Widget for WaterMarkTextBlock {
    fn create() -> Template {
        TextBlock::create()
            .with_property(WaterMark::from("Placeholder"))
            .with_property(Selector::from("watermark"))
            .with_state(Rc::new(WaterMarkTextBlockState::default()))
            .with_debug_name("WaterMarkTextBlock")
    }
}
