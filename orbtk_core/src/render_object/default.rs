use crate::{proc_macros::IntoRenderObject, render_object::*};

/// The `DefaultRenderObject` holds default objects inside the Render
/// Object
#[derive(Debug, IntoRenderObject)]
pub struct DefaultRenderObject;

impl RenderObject for DefaultRenderObject {}
