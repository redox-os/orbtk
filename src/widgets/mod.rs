//! Widgets are the primary elements to create user interfaces with orbtk.
//!
//! This module contains base structures to create widgets and a set of
//! default widgets.

use orbclient::Renderer;
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::fmt;

use events::{Event, KeyEventArgs, MouseEventArgs, MouseMoveEventArgs, ScrollEventArgs};
use rect::Rect;
use point::Point;
use theme::Theme;
use thickness::Thickness;

pub use self::button::Button;
pub use self::combo_box::ComboBox;
pub use self::label::Label;
pub use self::menu::{Action, Menu, Separator};
pub use self::progress_bar::ProgressBar;
pub use self::text_box::TextBox;
pub use self::list::{Entry, List};

mod button;
mod combo_box;
mod label;
mod menu;
mod progress_bar;
mod text_box;
mod list;

/// Describes the vertical placement of a widget.
#[derive(PartialEq, Copy, Clone)]
pub enum VerticalPlacement {
    Top,
    Center,
    Bottom,
    Absolute,
    Stretch,
}

/// Describes the horizontal placement of a widget.
#[derive(PartialEq, Copy, Clone)]
pub enum HorizontalPlacement {
    Left,
    Center,
    Right,
    Absolute,
    Stretch,
}

/// Represents the base of all widgets.
pub trait Widget: Any {
    /// Borrow the render rect of the widget.
    fn rect(&self) -> &Cell<Rect>;

    /// Borrow the local position of the widget. The local position describes position of the widget relative to it's parent.
    fn local_position(&self) -> &Cell<Point>;

    /// Borrow the vertical placement of the widget.
    fn vertical_placement(&self) -> &Cell<VerticalPlacement>;

    /// Borrow the horizontal placement of the widget.
    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement>;

    /// Borrow the margin of the widget.
    fn margin(&self) -> &Cell<Thickness>;

    /// Used to draw the widget by render code.
    fn draw(&self, _renderer: &mut Renderer, _theme: &Theme) {}

    fn on_preview_key_down(&self, _args: &KeyEventArgs) {}
    fn on_preview_key_up(&self, _args: &KeyEventArgs) {}
    fn on_preview_mouse_down(&self, _args: &MouseEventArgs) {}
    fn on_preview_mouse_up(&self, _args: &MouseEventArgs) {}
    fn on_preview_mouse_enter(&self, _args: &MouseMoveEventArgs){}
    fn on_preview_mouse_leave(&self, _args: &MouseMoveEventArgs){}
    fn on_preview_scroll(&self, _args: &ScrollEventArgs) {}

    fn on_key_down(&self, _args: &KeyEventArgs) {}
    fn on_key_up(&self, _args: &KeyEventArgs) {}
    fn on_mouse_down(&self, _args: &MouseEventArgs) {}
    fn on_mouse_up(&self, _args: &MouseEventArgs) {}
    fn on_mouse_enter(&self, _args: &MouseMoveEventArgs) {}
    fn on_mouse_leave(&self, _args: &MouseMoveEventArgs) {}
    fn on_scroll(&self, _args: &ScrollEventArgs) {}


    /// Return the name of the widget.
    fn name(&self) -> &str;

    /// Borrow the children of the widget.
    fn children(&self) -> &RefCell<Vec<Arc<Widget>>>;

    /// Add a child to the widget.
    fn add(&self, widget: Arc<Widget>) {
        (*self.children().borrow_mut()).push(widget);
        self.arrange();
    }

    /// Used to update the state of the widget. Could be used to update the selector.
    fn update(&self) {}

    /// Arrange the children of the widget. Could be override to create a custom layout.
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

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| - {}", self.name())
    }
}