use orbclient::Color;

const fn hex(data: u32) -> Color {
    Color { data: 0xFF000000 | data }
}

const BLACK: Color = hex(0x000000);
const SELECT_BLUE: Color = hex(0x5294E2);
const BORDER_GREY: Color = hex(0xCFD6E6);
const WINDOW_GREY: Color = hex(0xF5F6F7);
const BUTTON_WHITE: Color = hex(0xFBFBFC);
const WHITE: Color = hex(0xFFFFFF);

pub static WINDOW_BACKGROUND: Color = WINDOW_GREY;

pub static LABEL_BACKGROUND: Color = WINDOW_GREY;
pub static LABEL_BORDER: Color = BORDER_GREY;
pub static LABEL_FOREGROUND: Color = BLACK;

pub static BUTTON_BACKGROUND: Color = BUTTON_WHITE;
pub static BUTTON_BG_SELECTION: Color = SELECT_BLUE;
pub static BUTTON_BORDER: Color = BORDER_GREY;
pub static BUTTON_FOREGROUND: Color = BLACK;
pub static BUTTON_FG_SELECTION: Color = WHITE;

pub static ITEM_BACKGROUND: Color = WHITE;
pub static ITEM_BORDER: Color = BORDER_GREY;
pub static ITEM_FOREGROUND: Color = BLACK;
pub static ITEM_SELECTION: Color = SELECT_BLUE;

pub static TEXT_BACKGROUND: Color = WHITE;
pub static TEXT_BORDER: Color = BORDER_GREY;
pub static TEXT_FOREGROUND: Color = BLACK;
pub static TEXT_SELECTION: Color = SELECT_BLUE;
