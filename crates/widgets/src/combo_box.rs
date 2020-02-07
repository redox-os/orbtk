use std::cell::RefCell;

use crate::shell::CONSOLE;
use crate::{prelude::*, utils::SelectionMode as SelMode};

use super::behaviors::{MouseBehavior, SelectionBehavior};

/// The `ComboBoxState` is used to manipulate the position of the thumb of the slider widget.
#[derive(Default, AsAny)]
pub struct ComboBoxState {
    builder: RefCell<Vec<WidgetBuildContext>>,
    selected_item_builder: WidgetBuildContext,
}

impl State for ComboBoxState {
    // fn init(&mut self, _: Registry)
}

widget!(
    /// The `ComboBox` represents an selection widget with a drop-down list.
    ///
    /// **CSS element:** `ComboBox`
    ComboBox<ComboBoxState>: MouseHandler, ChangedHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shared the count.
        count: usize,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the selected index.
        selected_index: u32,

        /// Sets or shares selected entity.
        selected_entity: u32,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the flag if the drop down is open.
        selected: bool
    }
);

impl ComboBox {
    /// Creates a ComboBox and add the builder for the list items inside of the ComboBox.
    pub fn from_items_builder<F: Fn(&mut BuildContext, usize) -> Entity + 'static>(
        builder: F,
    ) -> Self {
        let mut combo_box = ComboBox::create();
        combo_box
            .state_mut()
            .builder
            .borrow_mut()
            .push(Some(Box::new(builder)));
        combo_box
    }

    /// Define the builder function for the selected item inside of the header of the ComboBox.
    pub fn selected_item_builder<F: Fn(&mut BuildContext, usize) -> Entity + 'static>(mut self, builder: F) -> Self {
        self.state_mut().selected_item_builder = Some(Box::new(builder));
        self
    }
}

impl Template for ComboBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let container = Container::create()
            .background(id)
            .border_radius(id)
            .border_width(id)
            .border_brush(id)
            .padding(id)
            .build(ctx);

        let mut list_view = ListView::create().count(id).selection_mode(SelMode::Single);

        // Workaround to move builder out of state
        if let Some(builder) = self.state().builder.borrow_mut().pop() {
            if let Some(builder) = builder {
                list_view = list_view.items_builder(builder);
            }
        }

        let list_view = list_view.build(ctx);

        let popup = Popup::create()
            .height(200.0)
            .open(("selected", id))
            .child(list_view)
            .target(container.0)
            .build(ctx);

        let result = ctx.append_child_to_overlay(popup);

        if let Err(e) = result {
            CONSOLE.log(format!("{:?}", e));
        }
        self.name("ComboBox")
            .selector("combo_box")
            .height(32.0)
            .min_width(80.0)
            .selected(false)
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
                            .parent(id.0)
                            .child(container)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
