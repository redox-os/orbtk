use crate::{api::prelude::*, proc_macros::*};

// Depreciated: use function layout()
// /// The `PopupAction` represent actions that can be sent to `PopupState`.
// pub enum PopupAction {
//     // placement of the popup relative to its target
//     UpdatePlacement,

//     // update the visiability status
//     UpdateVisibility,
// }

/// The `PopupState` handles the open and close behavior of the `Popup` widget.
#[derive(AsAny, Default)]
pub struct PopupState {
    //actions: Vec<PopupAction>,
}

impl PopupState {
    // Depreciated: use function layout()
    // /// If the placement property of the popup widget changes, its
    // /// bounds will be measured and the rendered widget will be placed
    // /// consuming the given position relative to its associated target.
    // pub fn update_placement(&mut self) {
    //	self.actions.push(PopupAction::UpdatePlacement);
    // }

    // Depreciated: use function layout()
    // /// the visibility property enables where do we place the popup
    // /// widget in relation to the given target.
    // pub fn update_visibility(&mut self) {
    //	self.actions.push(PopupAction::UpdateVisibility);
    // }

    // Deprecated: use function layout().
    // fn update_placement_internal(&mut self, _registry: &mut Registry, ctx: &mut Context) {
    //	if let Some(target) = ctx.widget().try_clone::<PopupTarget>("target") {
    //	    let current_bounds: Rectangle = ctx.widget().clone("bounds");
    //	    let current_constraint: Constraint = ctx.widget().clone("constraint");

    //	    let real_target_bounds = match target {
    //		PopupTarget::Entity(entity) => {
    //		    let target_position: Point = ctx.get_widget(entity).clone("position");

    //		    // WARNING: widget bounds (width an height values)
    //		    // will be measured as available space for the
    //		    // widget. Only in `post_layout_update` state it will
    //		    // reflect the effective size of the given entity.
    //		    let mut target_bounds: Rectangle = ctx.get_widget(entity).clone("bounds");
    //		    target_bounds.set_position(target_position);
    //		    target_bounds
    //		}
    //		PopupTarget::Point(mut point) => {
    //		    point.set_x(point.x() + current_bounds.width() / 2.0);
    //		    point.set_y(point.y() + current_bounds.height() / 2.0);
    //		    Rectangle::new(point, (0.0, 0.0))
    //		}
    //	    };

    //	    let placement: Placement = ctx.widget().clone_or_default("placement");

    //	    let new_popup_bounds = match placement {
    //		Placement::Left => {
    //		    let current_v_align: Alignment = ctx.widget().clone("v_align");

    //		    let x = real_target_bounds.x() - current_bounds.width();
    //		    let y = current_v_align.align_position(
    //			real_target_bounds.height(),
    //			current_bounds.height(),
    //			real_target_bounds.y(),
    //			real_target_bounds.y() + real_target_bounds.height(),
    //		    );

    //		    let width = current_bounds.width();
    //		    let height = current_v_align.align_measure(
    //			real_target_bounds.height(),
    //			current_bounds.height(),
    //			0.0,
    //			0.0,
    //		    );

    //		    Rectangle::new((x, y), current_constraint.perform((width, height)))
    //		}
    //		Placement::Right => {
    //		    let current_v_align: Alignment = ctx.widget().clone("v_align");

    //		    let x = real_target_bounds.x() + real_target_bounds.width();
    //		    let y = current_v_align.align_position(
    //			real_target_bounds.height(),
    //			current_bounds.height(),
    //			real_target_bounds.y(),
    //			real_target_bounds.y() + real_target_bounds.height(),
    //		    );

    //		    let width = current_bounds.width();
    //		    let height = current_v_align.align_measure(
    //			real_target_bounds.height(),
    //			current_bounds.height(),
    //			0.0,
    //			0.0,
    //		    );

    //		    Rectangle::new((x, y), current_constraint.perform((width, height)))
    //		}
    //		Placement::Top => {
    //		    let current_h_align: Alignment = ctx.widget().clone("h_align");

    //		    let x = current_h_align.align_position(
    //			real_target_bounds.width(),
    //			current_bounds.width(),
    //			real_target_bounds.x(),
    //			real_target_bounds.x() + real_target_bounds.width(),
    //		    );
    //		    let y = real_target_bounds.y() - current_bounds.height();
    //		    let width = current_h_align.align_measure(
    //			real_target_bounds.width(),
    //			current_bounds.width(),
    //			0.0,
    //			0.0,
    //		    );
    //		    let height = current_bounds.height();

    //		    Rectangle::new((x, y), current_constraint.perform((width, height)))
    //		}
    //		Placement::Bottom => {
    //		    let current_h_align: Alignment = ctx.widget().clone("h_align");

    //		    let x = current_h_align.align_position(
    //			real_target_bounds.width(),
    //			current_bounds.width(),
    //			real_target_bounds.x(),
    //			real_target_bounds.x() + real_target_bounds.width(),
    //		    );
    //		    let y = real_target_bounds.y() + real_target_bounds.height();
    //		    let width = current_h_align.align_measure(
    //			real_target_bounds.width(),
    //			current_bounds.width(),
    //			0.0,
    //			0.0,
    //		    );
    //		    let height = current_bounds.height();

    //		    Rectangle::new((x, y), current_constraint.perform((width, height)))
    //		}
    //	    };

    //	    ctx.widget().set::<Rectangle>("bounds", new_popup_bounds);
    // } else {
    //     println!("Target not found");
    // }
	//}

    // Deprecated: use function layout().
    // fn update_visibility_internal(&mut self, _registry: &mut Registry, ctx: &mut Context) {
    //	//if cfg!(feature = "log") {
    //	println!("popup2: update_visibility_internal: started");
    //	//}

    //	let mut widget = ctx.widget();
    //	let open = widget.clone::<bool>("open");
    //	let visibility: &mut Visibility = widget.get_mut("visibility");

    //	//if cfg!(feature = "log") {
    //	println!("popup2: Current status open: {:?}", open);
    //	println!("popup2: Current status Visibility: {:?}", visibility);
    //	//}

    //	match (open, *visibility) {
    //	    (true, Visibility::Visible) => {}
    //	    (true, Visibility::Hidden) => *visibility = Visibility::Visible,
    //	    (true, Visibility::Collapsed) => *visibility = Visibility::Visible,
    //	    (false, Visibility::Visible) => *visibility = Visibility::Collapsed,
    //	    (false, Visibility::Hidden) => *visibility = Visibility::Collapsed,
    //	    (false, Visibility::Collapsed) => {}
    //	}

    //	//if cfg!(feature = "log") {
    //	println!("popup2: Updated status open: {}", open);
    //	println!("popup2: Updated status visibility: {:?}", *visibility);
    //	println!("popup2: update_visibility_internal: finished");
    //	//}
    // }
}

impl State for PopupState {
    // fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
    //	let actions: Vec<PopupAction> = self.actions.drain(..).collect();
    //	for action in actions {
    //	    match action {
    //		PopupAction::UpdatePlacement => self.update_placement_internal(registry, ctx),
    //		PopupAction::UpdateVisibility => self.update_visibility_internal(registry, ctx),
    //	    }
    //	}
    // }
}

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
