use crate::{proc_macros::IntoRenderObject, render_object::*};

#[derive(Debug, IntoRenderObject)]
pub struct DefaultRenderObject;

impl RenderObject for DefaultRenderObject {}
