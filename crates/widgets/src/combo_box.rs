use crate::{prelude::*, utils::SelectionMode as SelMode};

/// The `ComboBoxState` is used to manipulate the position of the thumb of the slider widget.
#[derive(Default, AsAny)]
pub struct ComboBoxState {}

impl State for ComboBoxState {}

widget!(
    /// The `ComboBox` represents an selection widget with a drop-down list.
    ///
    /// **CSS element:** `ComboBox`
    ComboBox<ComboBoxState>: MouseHandler, ChangedHandler {
         /// Sets or shared the count.
         count: usize,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the selection mode property.
        selection_mode: SelectionMode,

        /// Sets or shares the selected indices.
        selected_indices: SelectedIndices,

        /// Sets or shares the list of selected indices.
        selected_entities: SelectedEntities
    }
);

impl Template for ComboBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ComboBox").selector("combo_box")
    }
}
