use enums::Visibility;
use std::rc::Rc;
use structs::{Label, WaterMark};
use theme::Selector;
use widget::{State, Template, TextBlock, Widget, WidgetContainer};

/// The `WaterMarkTextBlockState` handles the text processing of the `WaterMarkTextBlock` widget.
#[derive(Default)]
pub struct WaterMarkTextBlockState;

impl Into<Rc<State>> for WaterMarkTextBlockState {
    fn into(self) -> Rc<State> {
        Rc::new(self)
    }
}

impl State for WaterMarkTextBlockState {
    fn update(&self, widget: &mut WidgetContainer) {
        let mut is_label_empty = false;
        let mut is_water_mark_empty = false;

        if let Ok(label) = widget.borrow_property::<Label>() {
            is_label_empty = label.0.is_empty();
        }

        if let Ok(label) = widget.borrow_property::<WaterMark>() {
            is_water_mark_empty = label.0.is_empty();
        }

        if let Ok(visibility) = widget.borrow_mut_property::<Visibility>() {
            if is_water_mark_empty || !is_label_empty {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
            }
        }
    }
}

/// The `WaterMarkTextBlock` is used to display a placeholder watermark. If `Label` property is not empty the `WaterMarkTextBlock` is hidden.
pub struct WaterMarkTextBlock;

impl Widget for WaterMarkTextBlock {
    fn create() -> Template {
        TextBlock::create()
            .with_property(WaterMark::from("Placeholder"))
            .with_property(Selector::new().with("watermark"))
            .with_property(Visibility::Visible)
            .with_state(Rc::new(WaterMarkTextBlockState::default()))
            .with_debug_name("WaterMarkTextBlock")
    }
}
