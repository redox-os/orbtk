use orbclient::Renderer;
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use event::Event;
use rect::Rect;
use point::Point;
use theme::Theme;

pub use self::button::Button;
pub use self::combo_box::ComboBox;
pub use self::grid::Grid;
pub use self::label::Label;
pub use self::menu::{ Menu, Action, Separator };
pub use self::progress_bar::ProgressBar;
pub use self::text_box::TextBox;
pub use self::list::{ List, Entry };

pub use self::test_button::TestButton;

mod button;
mod combo_box;
mod grid;
mod label;
mod menu;
mod progress_bar;
mod text_box;
mod list;

mod test_button;

pub enum VerticalPlacement { 
    Top,
    Center,
    Bottom,
    Absolute,
}

pub enum HorizontalPlacement {
    Left,
    Center,
    Right,
    Absolute,
}

pub trait Widget : Any {
    fn rect(&self) -> &Cell<Rect>;
    fn local_position(&self) -> &Cell<Point>;
    fn vertical_placement(&self) -> &Cell<VerticalPlacement>;
    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement>;
    fn draw(&self, _renderer: &mut Renderer, _focused: bool, _theme: &Theme) {}
    fn event(&self, _event: Event, _focused: bool, _redraw: &mut bool) -> bool {
        _focused
    }
    fn name(&self) -> &str;
    fn children(&self) -> &RefCell<Vec<Arc<Widget>>>;
    fn arrage(&self) {

    }
}
