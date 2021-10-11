use std::{cell::RefCell, collections::BTreeMap};

use dces::prelude::*;

use crate::{
    prelude::*, proc_macros::IntoLayout, render::RenderContext2D, theming::*, tree::Tree,
    utils::prelude::*,
};

use super::{component, component_or_default, component_try_mut, try_component, Layout};

/// The `PopupLayout` handles measuring and arrangement of a `Popup` widget.
#[derive(Default, IntoLayout)]
pub struct PopupLayout {
    // TODO: Add padding to the widget.
    desired_size: RefCell<DirtySize>,
}

/// Associated functions, that react on any given state change inside the `Popup` widget.
impl PopupLayout {
    /// Instatiate a new PopupLayout object.
    pub fn new() -> Self {
	PopupLayout::default()
    }
}

impl Layout for PopupLayout {
    fn measure(
	&self,
	render_context_2_d: &mut RenderContext2D,
	entity: Entity,
	ecm: &mut EntityComponentManager<Tree>,
	layouts: &BTreeMap<Entity, Box<dyn Layout>>,
	theme: &Theme,
    ) -> DirtySize {
	// if visibility is collapsed, the entity is rendered but
	// shouldn't consume any size in the render buffer.
	if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
	    self.desired_size.borrow_mut().set_size(0.0, 0.0);
	    return *self.desired_size.borrow();
	}

	if let Some(target) = try_component::<PopupTarget>(ecm, entity, "target") {
	    let current_bounds: Rectangle = component(ecm, entity, "bounds");
	    let current_constraint: Constraint = component(ecm, entity, "constraint");

	    let real_target_bounds = match target {
		PopupTarget::Entity(entity) => {
		    let target_position: Point = component(ecm, entity, "position");

		    // WARNING: widget bounds (width an height values)
		    // will be measured as available space for the
		    // widget. Only in state `post_layout_update` it
		    // will reflect the effective size of the given
		    // entity.
		    let mut target_bounds: Rectangle = component(ecm, entity, "bounds");
		    target_bounds.set_position(target_position);
		    target_bounds
		}
		PopupTarget::Point(mut point) => {
		    point.set_x(point.x() + current_bounds.width() / 2.0);
		    point.set_y(point.y() + current_bounds.height() / 2.0);
		    Rectangle::new(point, (0.0, 0.0))
		}
	    };

	    let placement: Placement = component_or_default(ecm, entity, "placement");

	    let new_popup_size = match placement {
		Placement::Bottom => {
		    let current_h_align: Alignment = component(ecm, entity, "h_align");

		    let width = current_h_align.align_measure(
			real_target_bounds.width(),
			current_bounds.width(),
			0.0,
			0.0,
		    );
		    let height = current_bounds.height();

		    //let y = real_target_bounds.y() - current_bounds.height();

		    current_constraint.perform((width, height))
		}
		Placement::Left => {
		    let current_v_align: Alignment = component(ecm, entity, "v_align");

		    let width = current_bounds.width();
		    let height = current_v_align.align_measure(
			real_target_bounds.height(),
			current_bounds.height(),
			0.0,
			0.0,
		    );

		    // respect given bound constraints
		    current_constraint.perform((width, height))
		}
		Placement::Right => {
		    let current_v_align: Alignment = component(ecm, entity, "v_align");

		    let width = current_bounds.width();
		    let height = current_v_align.align_measure(
			real_target_bounds.height(),
			current_bounds.height(),
			0.0,
			0.0,
		    );

		    current_constraint.perform((width, height))
		}
		Placement::Top => {
		    let current_h_align: Alignment = component(ecm, entity, "h_align");

		    let width = current_h_align.align_measure(
			real_target_bounds.width(),
			current_bounds.width(),
			0.0,
			0.0,
		    );
		    let height = current_bounds.height();

		    current_constraint.perform((width, height))
		}
	    };

	    {
		let mut desired_size = self.desired_size.borrow_mut();
		desired_size.set_width(new_popup_size.0);
		desired_size.set_height(new_popup_size.1);
	    }

	    let padding: Thickness = component(ecm, entity, "padding");
	    for index in 0..ecm.entity_store().children[&entity].len() {
		let child = ecm.entity_store().children[&entity][index];

		if let Some(child_layout) = layouts.get(&child) {
		    let child_desired_size =
			child_layout.measure(render_context_2_d, child, ecm, layouts, theme);
		    let mut desired_size = self.desired_size.borrow().size();

		    let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();
		    self.desired_size.borrow_mut().set_dirty(dirty);

		    let child_margin = *ecm
			.component_store()
			.get::<Thickness>("margin", child)
			.unwrap();

		    desired_size.0 = desired_size.0.max(
			child_desired_size.width()
			    + padding.left()
			    + padding.right()
			    + child_margin.left()
			    + child_margin.right(),
		    );
		    desired_size.1 = desired_size.1.max(
			child_desired_size.height()
			    + padding.top()
			    + padding.bottom()
			    + child_margin.top()
			    + child_margin.left(),
		    );

		    self.desired_size
			.borrow_mut()
			.set_size(desired_size.0, desired_size.1);
		}
	    }
	} else {
	    println!("Target not found");
	}

	*self.desired_size.borrow()
    }

    fn arrange(
	&self,
	render_context_2_d: &mut RenderContext2D,
	parent_size: (f64, f64),
	entity: Entity,
	ecm: &mut EntityComponentManager<Tree>,
	layouts: &BTreeMap<Entity, Box<dyn Layout>>,
	theme: &Theme,
    ) -> (f64, f64) {
	if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
	    self.desired_size.borrow_mut().set_size(0.0, 0.0);
	    return (0.0, 0.0);
	}

	if !self.desired_size.borrow().dirty() {
	    return self.desired_size.borrow().size();
	}

	let horizontal_alignment: Alignment = component(ecm, entity, "h_align");
	let vertical_alignment: Alignment = component(ecm, entity, "v_align");
	let margin = *ecm
	    .component_store()
	    .get::<Thickness>("margin", entity)
	    .unwrap();
	let padding: Thickness = component(ecm, entity, "padding");
	let constraint: Constraint = component(ecm, entity, "constraint");

	let size = constraint.perform((
	    horizontal_alignment.align_measure(
		parent_size.0,
		self.desired_size.borrow().width(),
		margin.left(),
		margin.right(),
	    ),
	    vertical_alignment.align_measure(
		parent_size.1,
		self.desired_size.borrow().height(),
		margin.top(),
		margin.bottom(),
	    ),
	));

	if let Some(bounds) = component_try_mut::<Rectangle>(ecm, entity, "bounds") {
	    bounds.set_width(size.0);
	    bounds.set_height(size.1);
	}

	mark_as_dirty("bounds", entity, ecm);

	let available_size = (
	    size.0 - padding.left() - padding.right(),
	    size.1 - padding.top() - padding.bottom(),
	);

	for index in 0..ecm.entity_store().children[&entity].len() {
	    let child = ecm.entity_store().children[&entity][index];

	    let child_margin: Thickness = component(ecm, entity, "margin");

	    if let Some(child_layout) = layouts.get(&child) {
		child_layout.arrange(
		    render_context_2_d,
		    available_size,
		    child,
		    ecm,
		    layouts,
		    theme,
		);
	    }

	    let child_horizontal_alignment: Alignment =
		*ecm.component_store().get("h_align", child).unwrap();
	    let child_vertical_alignment: Alignment =
		*ecm.component_store().get("v_align", child).unwrap();
	    if let Ok(child_bounds) = ecm
		.component_store_mut()
		.get_mut::<Rectangle>("bounds", child)
	    {
		child_bounds.set_x(
		    padding.left()
			+ child_horizontal_alignment.align_position(
			    available_size.0,
			    child_bounds.width(),
			    child_margin.left(),
			    child_margin.right(),
			),
		);
		child_bounds.set_y(
		    padding.top()
			+ child_vertical_alignment.align_position(
			    available_size.1,
			    child_bounds.height(),
			    child_margin.top(),
			    child_margin.bottom(),
			),
		);
	    }

	    mark_as_dirty("bounds", child, ecm);
	}

	self.desired_size.borrow_mut().set_dirty(false);
	size
    }
}
