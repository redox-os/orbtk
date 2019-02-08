use crate::{
    enums::ParentType,
    layout::StackLayout,
    properties::OrientationProperty,
    widget::{Template, Widget},
};

/// The `Stack` represents a layout widget that is used to stack its children on the z-axis.
///
/// # Properties
///
/// * `orientation` - used to define the orientation of the stack layout vertical (default) of horizontal.
/// 
/// # Others
///
/// * `ParentType`- Mutli.
/// * `StretchLayout` - Used to layout the widget.
pub struct Stack;

impl Widget for Stack {
    type Template = StackTemplate;

    fn create() -> Self::Template {
        StackTemplate::new()
            .orientation("Vertical")
            .parent_type(ParentType::Multi)
            .layout(StackLayout::new())
            .debug_name("Stack")
    }
}

template!(StackTemplate, [OrientationProperty]);
