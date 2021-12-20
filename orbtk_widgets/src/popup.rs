use crate::{api::prelude::*, proc_macros::*};

/// The `PopupState` handles the open and close behavior of the `Popup` widget.
#[derive(AsAny, Default)]
pub struct PopupState {
    //actions: Vec<PopupAction>,
}

impl PopupState {}

impl State for PopupState {}

widget!(
    /// The `Popup` is used to presents content bound to target entity.
    ///
    /// The `target` is specified either via its widget id (`Entitiy`)
    /// or using a point coordinate (`Point`). The placmement of the
    /// popup widget itself is controlled via its `placement`
    /// property. An optional attribute (float), defines the
    /// margin between the target and the popup widget.
    ///
    /// [`placement`]: ../orbtk_core/render_object/enum.Placement.html
    ///
    /// **style:** `popup``
    Popup<PopupState> : KeyDownHandler, MouseHandler {
    /// Sets or shares the background property.
    background: Brush,

    /// Sets or shares the border brush property.
    border_brush: Brush,

    /// Sets or shares the border radius property.
    border_radius: f64,

    /// Sets or shares the border thickness property.
    border_width: Thickness,

    /// Sets or shares the popup open state.
    open: bool,

    /// Sets or shares the padding property.
    padding: Thickness,

    /// Sets or shares the placement property relative to the
    /// target position. Valid placement variants are defined via
    /// the `Placement` enumeration.
    placement: Placement,

    /// Sets or shares the offset property that assignes a margin
    /// between popup and target entity.
    offset: f64,

    ///
    /// Defined ether as an entity (Entity), or as a point
    /// coordinate (Point).
    target: PopupTarget
    }
);

impl Template for Popup {
    fn template(self, _id: Entity, _: &mut BuildContext) -> Self {
        self.name("Popup").style("popup").open(false)
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(PopupRenderObject::new())
    }

    fn layout(&self) -> Box<dyn Layout> {
        PopupLayout::new().into()
    }
}
