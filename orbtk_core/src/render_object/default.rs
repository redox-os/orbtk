use crate::{proc_macros::IntoRenderObject, render_object::*};

/// The `DefaultRenderObject` holds default objects inside
/// a render object.
#[derive(Debug, IntoRenderObject)]
pub struct DefaultRenderObject;

impl RenderObject for DefaultRenderObject {}
