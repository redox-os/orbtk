use orbclient::Renderer;
use std::any::Any;
use std::cell::Cell;

use event::Event;
use rect::Rect;

pub use self::button::Button;
pub use self::grid::Grid;
pub use self::image::Image;
pub use self::label::Label;
pub use self::menu::{ Menu, Action, Separator };
pub use self::progress_bar::ProgressBar;
pub use self::text_box::TextBox;
pub use self::list::{ List, Entry };
pub use self::control_knob::ControlKnob;
pub use self::toolbar::{Toolbar,ToolbarIcon};
pub use self::color_swatch::ColorSwatch;
pub use self::marquee::Marquee;

mod button;
mod grid;
mod image;
mod label;
mod menu;
mod progress_bar;
mod text_box;
mod list;
mod control_knob;
mod toolbar;
mod color_swatch;
mod marquee;

pub trait Widget : Any {
    fn rect(&self) -> &Cell<Rect>;
    fn draw(&self, renderer: &mut Renderer, focused: bool);
    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool;
    fn visible(&self, flag: bool);
}

pub trait ToolbarWidget : Any {
    fn selected(&self, flag: bool);
}
