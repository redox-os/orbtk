use crate::{
    render_object::*,
    utils::Point,
};

/// The target of the popup, given as an entity or as fixed point coordinate.
#[derive(Clone, Debug, PartialEq)]
pub enum PopupTarget {
    /// the entity of the target (Entity).
    Entity(Entity),

    /// the point coordinate (f64, f64).
    Point(Point),
}

impl Default for PopupTarget {
    fn default() -> Self {
	Self::Point(Point::new(0.0, 0.0))
    }
}

impl From<Entity> for PopupTarget {
    fn from(entity: Entity) -> Self {
	Self::Entity(entity)
    }
}

impl From<Point> for PopupTarget {
    fn from(point: Point) -> Self {
	Self::Point(point)
    }
}

impl IntoPropertySource<PopupTarget> for Entity {
    fn into_source(self) -> PropertySource<PopupTarget> {
	PropertySource::Value(self.into())
    }
}

impl IntoPropertySource<PopupTarget> for Point {
    fn into_source(self) -> PropertySource<PopupTarget> {
	PropertySource::Value(self.into())
    }
}

/// Defines popup placement options relative to its target.
#[derive(Clone, Debug, PartialEq)]
pub enum Placement {
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    Top,
    TopLeft,
    TopRight,
}

impl Default for Placement {
    fn default() -> Self {
	Self::Bottom
    }
}
impl Placement {
    /// Returns the placement name.
    pub fn get_name(&self, index: i32) -> String {
	match index {
	    0 => "Bottom".to_string(),
	    1 => "BottomLeft".to_string(),
	    2 => "BottomRight".to_string(),
	    3 => "Left".to_string(),
	    4 => "Right".to_string(),
	    5 => "Top".to_string(),
	    6 => "TopLeft".to_string(),
	    7 => "TopRight".to_string(),
	    _ => {
		eprintln!("popup: placement variant with index {} not coverd!",
			  index);
		std::process::exit(1);
	    }
	}
    }

    /// Returns the placement index.
    pub fn get_index(&self, string: &str) -> u32  {
	match string {
	    "Bottom" => 0,
	    "BottomLeft" => 1,
	    "BottomRight" => 2,
	    "Left" => 3,
	    "Right" => 4,
	    "Top" => 5,
	    "TopLeft" => 6,
	    "TopRight" => 7,
	    _ => {
		eprintln!("popup: placement variant {} not coverd!",
			  string);
		std::process::exit(1);
	    }
	}
    }

    /// Place the popup widget relative to the `bottom` of its parent target.
    pub fn bottom(self) -> Self {
	Self::Bottom
    }

    /// Place the popup widget relative to the `bottom` and `left` edge of its parent target.
    pub fn bottom_left(self) -> Self {
	Self::BottomLeft
    }

    /// Place the popup widget relative to the `bottom` and `right` edge of its parent target.
    pub fn bottom_right(self) -> Self {
	Self::BottomRight
    }

    /// Place the popup widget relative to the left of its parent target.
    pub fn left(self) -> Self {
	Self::Left
    }

    /// Place the popup widget relative to the `right` of its parent target.
    pub fn right(self) -> Self {
	Self::Right
    }
    /// Place the popup widget relative to the `top` of its parent target.
    pub fn top(self) -> Self {
	Self::Top
    }

    /// Place the popup widget relative to the `top` of its parent target.
    /// Place the popup widget relative to the `top` and `left` edge of its parent target.
    pub fn top_left(self) -> Self {
	Self::TopLeft
    }

    /// Place the popup widget relative to the `top` and `right` edge of its parent target.
    pub fn top_right(self) -> Self {
	Self::TopRight
    }
}

into_property_source!(Placement);

/// The `PopupRenderObject` is used to render the contents of a `Popup` widget inside a recangle.
pub struct PopupRenderObject(RectangleRenderObject);

impl PopupRenderObject {
    pub fn new() -> Self {
	Self::default()
    }
}
impl Default for PopupRenderObject {
    fn default() -> Self {
	Self(RectangleRenderObject)
    }
}

impl Into<Box<dyn RenderObject>> for PopupRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
	Box::new(self)
    }
}

impl RenderObject for PopupRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
	if let Some(target) = ctx.widget().try_clone::<PopupTarget>("target") {
	    let current_bounds: Rectangle = ctx.widget().clone("bounds");
	    let current_constraint: Constraint = ctx.widget().clone("constraint");

	    let target_bounds = match target {
		PopupTarget::Entity(entity) => {
		    let target_position: Point = ctx.get_widget(entity).clone("position");

		    // WARNING: target_bounds are only reflecting the
		    // actual effective size, if refered in
		    // post_layout_update function. Otherwise the
		    // bounds will refere to the space bounds
		    // available to the given widget.
		    let mut target_bounds: Rectangle = ctx.get_widget(entity).clone("bounds");
		    target_bounds.set_position(target_position);
		    target_bounds
		}
		PopupTarget::Point(mut point) => {
		    point.set_x(point.x() + current_bounds.width() / 2.0);
		    point.set_y(point.y() + current_bounds.height() / 2.0);
		    Rectangle::new(point, (0.0, 0.0))
		}
	    };

	    let placement: Placement = ctx.widget().clone_or_default("placement");
	    let offset: f64 = ctx.widget().clone_or_default("offset");

	    let new_popup_bounds = match placement {
		Placement::Left => {
		    let current_v_align: Alignment = ctx.widget().clone("v_align");

		    let x = target_bounds.x() - current_bounds.width() - offset;
		    let y = current_v_align.align_position(
			target_bounds.height(),
			current_bounds.height(),
			target_bounds.y(),
			target_bounds.y() + target_bounds.height(),
		    );

		    let width = current_bounds.width();
		    let height = current_v_align.align_measure(
			target_bounds.height(),
			current_bounds.height(),
			0.0,
			0.0,
		    );

		    Rectangle::new((x, y), current_constraint.perform((width, height)))
		}
		Placement::Right => {
		    let current_v_align: Alignment = ctx.widget().clone("v_align");

		    let x = target_bounds.x() + target_bounds.width() + offset;
		    let y = current_v_align.align_position(
			target_bounds.height(),
			current_bounds.height(),
			target_bounds.y(),
			target_bounds.y() + target_bounds.height(),
		    );

		    let width = current_bounds.width();
		    let height = current_v_align.align_measure(
			target_bounds.height(),
			current_bounds.height(),
			0.0,
			0.0,
		    );

		    Rectangle::new((x, y), current_constraint.perform((width, height)))
		}
		Placement::Top => {
		    let current_h_align: Alignment = ctx.widget().clone("h_align");

		    let x = current_h_align.align_position(
			target_bounds.width(),
			current_bounds.width(),
			target_bounds.x(),
			target_bounds.x() + target_bounds.width(),
		    );
		    let y = target_bounds.y() - current_bounds.height() - offset;
		    let width = current_h_align.align_measure(
			target_bounds.width(),
			current_bounds.width(),
			0.0,
			0.0,
		    );
		    let height = current_bounds.height();

		    Rectangle::new((x, y), current_constraint.perform((width, height)))
		}
		Placement::TopLeft => {
		    let current_h_align: Alignment = ctx.widget().clone("h_align");

		    let x = current_h_align.align_position(
			target_bounds.width(),
			current_bounds.width(),
			target_bounds.x(),
			target_bounds.x() + target_bounds.width(),
		    );
		    let y = target_bounds.y() - current_bounds.height() - offset;
		    let width = current_h_align.align_measure(
			target_bounds.width(),
			current_bounds.width(),
			0.0,
			0.0,
		    );
		    let height = current_bounds.height();

		    Rectangle::new((x, y), current_constraint.perform((width, height)))
		}
		Placement::TopRight => {
		    let current_h_align: Alignment = ctx.widget().clone("h_align");

		    let x = current_h_align.align_position(
			target_bounds.width(),
			current_bounds.width(),
			target_bounds.x(),
			target_bounds.x() + target_bounds.width(),
		    );
		    let y = target_bounds.y() - current_bounds.height() - offset;
		    let width = current_h_align.align_measure(
			target_bounds.width(),
			current_bounds.width(),
			0.0,
			0.0,
		    );
		    let height = current_bounds.height();

		    Rectangle::new((x, y), current_constraint.perform((width, height)))
		}
		Placement::Bottom => {
		    let current_h_align: Alignment = ctx.widget().clone("h_align");

		    let x = current_h_align.align_position(
			target_bounds.width(),
			current_bounds.width(),
			target_bounds.x(),
			target_bounds.x() + target_bounds.width(),
		    );
		    let y = target_bounds.y() + target_bounds.height() + offset;
		    let width = current_h_align.align_measure(
			target_bounds.width(),
			current_bounds.width(),
			0.0,
			0.0,
		    );
		    let height = current_bounds.height();

		    Rectangle::new((x, y), current_constraint.perform((width, height)))
		}
		Placement::BottomLeft => {
		    let current_h_align: Alignment = ctx.widget().clone("h_align");

		    let x = current_h_align.align_position(
			target_bounds.width(),
			current_bounds.width(),
			target_bounds.x(),
			target_bounds.x() + target_bounds.width(),
		    );
		    let y = target_bounds.y() + target_bounds.height() + offset;
		    let width = current_h_align.align_measure(
			target_bounds.width(),
			current_bounds.width(),
			0.0,
			0.0,
		    );
		    let height = current_bounds.height();

		    Rectangle::new((x, y), current_constraint.perform((width, height)))
		}
		Placement::BottomRight => {
		    let current_h_align: Alignment = ctx.widget().clone("h_align");

		    let x = current_h_align.align_position(
			target_bounds.width(),
			current_bounds.width(),
			target_bounds.x(),
			target_bounds.x() + target_bounds.width(),
		    );
		    let y = target_bounds.y() + target_bounds.height() + offset;
		    let width = current_h_align.align_measure(
			target_bounds.width(),
			current_bounds.width(),
			0.0,
			0.0,
		    );
		    let height = current_bounds.height();

		    Rectangle::new((x, y), current_constraint.perform((width, height)))
		}
	    };

	    ctx.widget().set::<Rectangle>("bounds", new_popup_bounds);
	} else {
	    println!("Target not found");
	}

	self.0.render_self(ctx, global_position);
    }
}
