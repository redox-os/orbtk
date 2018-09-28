#![crate_name = "orbtk"]
#![crate_type = "lib"]
#![deny(warnings)]
#![feature(const_fn)]

extern crate dces;

pub use dces::prelude::*;

pub use ComponentBox as Property;

extern crate cssparser;
extern crate orbclient;
extern crate orbfont;
extern crate orbimage;
#[macro_use]
extern crate lazy_static;

pub use orbclient::color::Color;

pub use application::*;
pub use backend::*;
pub use enums::*;
pub use layout_object::*;
pub use render_object::*;
pub use cell::CloneCell;
pub use error::*;
pub use structs::*;
pub use systems::*;
pub use theme::{Selector, Theme};
pub use tree::*;
pub use widget::*;

pub mod application;
pub mod backend;
pub mod enums;
pub mod layout_object;
pub mod render_object;
pub mod cell;
pub mod error;
pub mod structs;
pub mod systems;
pub mod theme;
pub mod tree;
pub mod widget;
