use crate::{prelude::*, render};
// #[derive(Clone, PartialEq)]
// pub struct TestRen {}

// impl RenderPipeline for TestRen {
//     fn box_eq(&self, other: &dyn Any) -> bool {
//         other.downcast_ref::<Self>().map_or(false, |a| self == a)
//     }
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn clone_box(&self) -> Box<RenderPipeline> {
//         Box::new(self.clone())
//     }
//     fn update(&self) {}
    
//     fn draw(
//         &self,
//         buffer: &mut crate::render::three::buffer::Buffer2d<f64>,
//         depth: &mut crate::render::three::buffer::Buffer2d<f64>,
//     ) {
//     }
// }



property!(
    /// 3D render pipeline to render 3D objects.
    RenderPipeline(Box<dyn render::RenderPipeline>)
);