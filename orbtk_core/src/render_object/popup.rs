use crate::{
    render_object::*,
    utils::{Point, Rectangle},
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
        Self::Point(Point::new(0.0, 30.0))
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

/// Popups placement relative to the target.
///
/// An otional distance attribute (float), defines the margin
/// between the selection box and the drop down popup.
#[derive(Clone, Debug, PartialEq)]
pub enum Placement {
    Top(f64),
    Bottom(f64),
    Left(f64),
    Right(f64),
}

impl Default for Placement {
    fn default() -> Self {
        Self::Bottom(5.0)
    }
}
impl Placement {
    /// Calculate the `margin` between popup and parent target (64bit float).
    pub fn get_margin(&self) -> f64 {
        match self {
            Self::Top(margin) => *margin,
            Self::Bottom(margin) => *margin,
            Self::Left(margin) => *margin,
            Self::Right(margin) => *margin,
        }
    }

    /// Place the popup widget relative to the `top` of its parent target.
    /// An optional `margin` adds a distance between popup and parent target.
    // before the function was called into_top()
    pub fn to_top(self) -> Self {
        Self::Top(self.get_margin())
    }

    /// Place the popup widget relative to the `bottom` of its parent target.
    /// An optional `margin` adds a distance between popup and parent target.
    pub fn to_bottom(self) -> Self {
        Self::Bottom(self.get_margin())
    }
    /// Place the popup widget relative to the left of its parent target.
    /// An optional `margin` adds a distance between popup and parent target.
    pub fn to_left(self) -> Self {
        Self::Left(self.get_margin())
    }
    /// Place the popup widget relative to the `right` of its parent target.
    /// An optional `margin` adds a distance between popup and parent target.
    pub fn to_right(self) -> Self {
        Self::Right(self.get_margin())
    }
}

/*
impl From<usize> for Placement {
    fn from(index: usize) -> Self {
        match entity
        {
            0 => Self::Top,
            1 => Self::Botton,
            2 => Self::Left,
            3 => Self::Right,
            _ => panic!()
        }
    }
}
*/
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

            // println!("render popup: bounds.width={:?} bounds.height={:?}",
            //          current_bounds.width(),
            //          current_bounds.height());

            let real_target_bounds = match target {
                PopupTarget::Entity(entity) => {
                    let target_position: Point = ctx.get_widget(entity.into()).clone("position");

                    //WARNING: this is true only if called during post_layout_update, otherwise the bounds will refere to space available to the widget, not the effective size
                    let mut target_bounds: Rectangle =
                        ctx.get_widget(entity.into()).clone("bounds");
                    target_bounds.set_position(target_position);
                    target_bounds
                }
                PopupTarget::Point(mut point) => {
                    point.set_x(point.x() + current_bounds.width() / 2.0);
                    point.set_y(point.y() + current_bounds.height() / 2.0);
                    Rectangle::new(point, (0.0, 0.0))
                }
            };

            let relative_position: Placement =
                ctx.widget().clone_or_default("relative_position");

            let new_popup_bounds = match relative_position {
                Placement::Left(margin) => {
                    let current_v_align: Alignment = ctx.widget().clone("v_align");

                    let x = real_target_bounds.x() - current_bounds.width() - margin;
                    let y = current_v_align.align_position(
                        real_target_bounds.height(),
                        current_bounds.height(),
                        real_target_bounds.y(),
                        real_target_bounds.y() + real_target_bounds.height(),
                    );

                    let width = current_bounds.width();
                    let height = current_v_align.align_measure(
                        real_target_bounds.height(),
                        current_bounds.height(),
                        0.0,
                        0.0,
                    );

                    Rectangle::new((x, y), current_constraint.perform((width, height)))
                }
                Placement::Right(margin) => {
                    let current_v_align: Alignment = ctx.widget().clone("v_align");

                    let x = real_target_bounds.x() + real_target_bounds.width() + margin;
                    let y = current_v_align.align_position(
                        real_target_bounds.height(),
                        current_bounds.height(),
                        real_target_bounds.y(),
                        real_target_bounds.y() + real_target_bounds.height(),
                    );

                    let width = current_bounds.width();
                    let height = current_v_align.align_measure(
                        real_target_bounds.height(),
                        current_bounds.height(),
                        0.0,
                        0.0,
                    );

                    Rectangle::new((x, y), current_constraint.perform((width, height)))
                }
                Placement::Top(margin) => {
                    let current_h_align: Alignment = ctx.widget().clone("h_align");

                    let x = current_h_align.align_position(
                        real_target_bounds.width(),
                        current_bounds.width(),
                        real_target_bounds.x(),
                        real_target_bounds.x() + real_target_bounds.width(),
                    );
                    let y = real_target_bounds.y() - current_bounds.height() - margin;
                    let width = current_h_align.align_measure(
                        real_target_bounds.width(),
                        current_bounds.width(),
                        0.0,
                        0.0,
                    );
                    let height = current_bounds.height();

                    Rectangle::new((x, y), current_constraint.perform((width, height)))
                }
                Placement::Bottom(margin) => {
                    let current_h_align: Alignment = ctx.widget().clone("h_align");

                    let x = current_h_align.align_position(
                        real_target_bounds.width(),
                        current_bounds.width(),
                        real_target_bounds.x(),
                        real_target_bounds.x() + real_target_bounds.width(),
                    );
                    let y = real_target_bounds.y() + real_target_bounds.height() + margin;
                    let width = current_h_align.align_measure(
                        real_target_bounds.width(),
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
