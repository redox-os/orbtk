use super::Point;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Init,

    Mouse {
        point: Point,
        left_button: bool,
        middle_button: bool,
        right_button: bool,
    },

    Scroll {
        x: i32,
        y: i32,
    },

    Text {
        c: char,
    },
    Enter,

    Backspace,
    Delete,

    Home,
    End,

    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,

    Resize {
        width: u32,
        height: u32,
    },

    Unknown,
}
