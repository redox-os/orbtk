use orbclient::Renderer;
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use event::Event;
use rect::Rect;
use point::Point;
use theme::Theme;
use thickness::Thickness;

pub use self::button::Button;
pub use self::combo_box::ComboBox;
pub use self::grid::Grid;
pub use self::label::Label;
pub use self::menu::{Action, Menu, Separator};
pub use self::progress_bar::ProgressBar;
pub use self::text_box::TextBox;
pub use self::list::{Entry, List};

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

#[derive(PartialEq, Copy, Clone)]
pub enum VerticalPlacement {
    Top,
    Center,
    Bottom,
    Absolute,
    Stretch,
}

#[derive(PartialEq, Copy, Clone)]
pub enum HorizontalPlacement {
    Left,
    Center,
    Right,
    Absolute,
    Stretch,
}

pub trait Widget: Any {
    fn rect(&self) -> &Cell<Rect>;
    fn local_position(&self) -> &Cell<Point>;
    fn vertical_placement(&self) -> &Cell<VerticalPlacement>;
    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement>;
    fn margin(&self) -> &Cell<Thickness>;
    fn draw(&self, _renderer: &mut Renderer, _focused: bool, _theme: &Theme) {}
    fn event(&self, _event: Event, _focused: bool, _redraw: &mut bool) -> bool {
        _focused
    }
    fn name(&self) -> &str;
    fn children(&self) -> &RefCell<Vec<Arc<Widget>>>;
    fn add(&self, widget: Arc<Widget>) {
        (*self.children().borrow_mut()).push(widget);
    }

    fn arrange(&self) {
        let parent_rect = self.rect().get();

        for child in &*self.children().borrow_mut() {
            let mut child_rect = child.rect().get();
            let child_position = child.local_position().get();
            let margin = child.margin().get();

            match child.vertical_placement().get() {
                VerticalPlacement::Absolute => {
                    child_rect.y = parent_rect.y + child_position.y;
                }
                VerticalPlacement::Stretch => {
                    child_rect.height =
                        parent_rect.height - margin.top as u32 - margin.bottom as u32;
                    child_rect.y = parent_rect.y + margin.top;
                }
                VerticalPlacement::Top => {
                    child_rect.y = parent_rect.y + margin.top;
                }
                VerticalPlacement::Center => {
                    child_rect.y = parent_rect.y + parent_rect.height as i32 / 2
                        - child_rect.height as i32 / 2;
                }
                VerticalPlacement::Bottom => {
                    child_rect.y = parent_rect.y + parent_rect.height as i32 - margin.bottom
                        - child_rect.height as i32;
                }
            }

            match child.horizontal_placement().get() {
                HorizontalPlacement::Absolute => {
                    child_rect.x = parent_rect.x + child_position.x;
                }
                HorizontalPlacement::Stretch => {
                    child_rect.width =
                        parent_rect.width - margin.left as u32 - margin.right as u32;
                    child_rect.x = parent_rect.x + margin.left;
                }
                HorizontalPlacement::Left => {
                    child_rect.x = parent_rect.x + margin.left;
                }
                HorizontalPlacement::Center => {
                    child_rect.x = parent_rect.x + parent_rect.width as i32 / 2
                        - child_rect.width as i32 / 2;
                }
                HorizontalPlacement::Right => {
                    child_rect.x = parent_rect.x + parent_rect.width as i32 - margin.right
                        - child_rect.width as i32;
                }
            }

            child.rect().set(child_rect);
            child.arrange();
        }
    }
}
