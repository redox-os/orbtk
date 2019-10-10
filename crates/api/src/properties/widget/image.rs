use std::{any::Any, fmt};

use crate::{prelude::*, render::Image as Img};

property!(
    /// `Image` describes an image property of a widget.
    #[derive(Default)]
    Image(Img) : &str,
    String
);

impl Image {
    pub fn width(&self) -> f64 {
        self.0.width()
    }

    pub fn height(&self) -> f64 {
        self.0.height()
    }
}

// pub trait TreeRender : Any {
//     fn box_eq(&self, other: &dyn Any) -> bool;
//     fn as_any(&self) -> &dyn Any;  
//     fn clone_box(&self) -> Box<TreeRender>;
//     // fn draw(&self, buffer: &mut crate::render::three::buffer::Buffer2d<f64>, depth: &mut crate::render::three::buffer::Buffer2d<f64>);
// }

// impl PartialEq for Box<dyn TreeRender> {
//     fn eq(&self, other: &Box<dyn TreeRender>) -> bool {
//         self.box_eq(other.as_any())
//     }
// }

// impl Clone for Box<dyn TreeRender> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }

// impl fmt::Debug for Box<dyn TreeRender> {
//      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Box<TreeRender>")
//     }
// }

// // pub struct Blub<T> where T: Default + Clone + PartialEq {
// //     what: T
// // }

// // impl Default for Box<dyn TreeRender> {
// //     fn default() -> Self { 
// //         Box::default()
// //     }
// // }


// property!(TreeRen(Box<dyn TreeRender>));