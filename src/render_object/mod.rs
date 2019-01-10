//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use std::collections::BTreeMap;

use crate::core::Renderer;
use crate::properties::Point;
use crate::widget::Context;

pub use self::font_icon::FontIconRenderObject;
pub use self::image::ImageRenderObject;
pub use self::rectangle::RectangleRenderObject;
pub use self::text::TextRenderObject;

mod font_icon;
mod image;
mod rectangle;
mod text;

use crate::core::{FillRule, Instruction};

// todos
//
// Step One Render Object refactoring
//
// *[*] Rename backend to core
// *[*] Add render context sub module to core (enums, traits, basis struct)
// *[*] Move Render Context related stuff to render context module
// *[ ] Move render shapes to folders
// *[ ] Implement render shapes: Rectangle, Text, Image, Canvas
// *[ ] Canvas widgets needs something like instruction builder for the user
// *[ ] Think about how to implement shadows
// *[ ] Adjust Template to work with new render objects e.g. .with_render_shape(RectangleBuilder::new())
// *[ ] Store render shapes in ecs
// *[ ] Implement RenderContext for OrbClient
// *[ ] Cache images
// *[ ] Remove old Renderer trait
// *[ ] Adjust Render system to work with render context trait
// *[ ] Write tests for e.g. render shapes
// *[ ] Test everthing
// *[ ] MR
//
// Step Two Expand Window
//
// *[ ] is_resizable
// *[ ] min_size
// *[ ] max_size
// *[ ] fix position on windows
// *[ ] Eventuelly refactor backend struct
// *[ ] Test everything
// *[ ] MR
//
// Step Three Web backend
//
// *[ ] implement web backend
// *[ ] Test everything
// *[ ] MR
//
// Step Four switch to OrbGL for OrbClient backend
//
// *[ ] convert more drawing functions to OrbGl in OrbClient backend
// *[ ] Test everything
// *[ ] MR
//
// Optional
//
// *[ ] Switch to winit for default client

pub struct Text {
    instructions: Vec<Instruction>,
}

pub struct Rectangle {
    instructions: Vec<Instruction>,
    // todo: option instruction without border /
}



// pub trait Shape2DBuilder {
//     fn build(&self) -> Box<Shape2D>;
// }

pub struct RectangleBuilder {}

// zoom, rotate, ...

pub struct CCanvas {}

pub trait RenderObject {
    fn render(
        &self,
        renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    );
}
