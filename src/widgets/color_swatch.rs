use orbclient::{Color, Renderer};
use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::{CloneCell, CheckSet};
use event::Event;
use point::Point;
use rect::Rect;
use theme::{BUTTON_BACKGROUND, BUTTON_BG_SELECTION, BUTTON_FOREGROUND, BUTTON_FG_SELECTION, BUTTON_BORDER};
use traits::{Border, Click, Place, Text};
use widgets::Widget;

pub struct ColorSwatch {
    pub rect: Cell<Rect>,
    pub bg: Cell<Color>,
    pub bg_selected: Color,
    pub fg: Color,
    pub fg_selected: Color,
    pub fg_border: Color,
    pub border: Cell<bool>,
    pub border_radius: Cell<u32>,
    pub text: CloneCell<String>,
    pub text_offset: Cell<Point>,
    click_callback: RefCell<Option<Arc<Fn(&ColorSwatch, Point)>>>,
    pressed: Cell<bool>,
    pub visible: Cell<bool>,
}

impl ColorSwatch {
    pub fn new() -> Arc<Self> {
        Arc::new(ColorSwatch {
            rect: Cell::new(Rect::default()),
            bg: Cell::new(BUTTON_BACKGROUND),
            bg_selected: BUTTON_BG_SELECTION,
            fg: BUTTON_FOREGROUND,
            fg_selected: BUTTON_FG_SELECTION,
            fg_border: BUTTON_BORDER,
            border: Cell::new(true),
            border_radius: Cell::new(2),
            text: CloneCell::new(String::new()),
            text_offset: Cell::new(Point::default()),
            click_callback: RefCell::new(None),
            pressed: Cell::new(false),
            visible: Cell::new(true),
        })
    }
    
    pub fn color(&self, color: Color) {
        self.bg.set(color);
    }
    pub fn read(&self) -> Color {
        self.bg.get()
    }
}

impl Border for ColorSwatch {
    fn border(&self, enabled: bool) -> &Self {
        self.border.set(enabled);
        self
    }

    fn border_radius(&self, radius: u32) -> &Self {
        self.border_radius.set(radius);
        self
    }
}

impl Click for ColorSwatch {
    fn emit_click(&self, point: Point) {
        if let Some(ref click_callback) = *self.click_callback.borrow() {
            click_callback(self, point);
        }
    }

    fn on_click<T: Fn(&Self, Point) + 'static>(&self, func: T) -> &Self {
        *self.click_callback.borrow_mut() = Some(Arc::new(func));
        self
    }
}

impl Place for ColorSwatch {}

impl Text for ColorSwatch {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self
    }
}

impl Widget for ColorSwatch {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        if self.visible.get(){
            let rect = self.rect.get();

            let w = rect.width as i32;
            let h = rect.height as i32;

            /*
            let (fg, bg) = if self.pressed.get() {
                (self.fg_selected, self.bg_selected)
            } else {
                (self.fg, self.bg)
            };
            */
            
            let b_r = self.border_radius.get();

            renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, true, self.bg.get());

            if self.border.get() {
                renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, false, self.fg_border);
            }

            let text = self.text.borrow();

            let mut point = self.text_offset.get();
            for c in text.chars() {
                if c == '\n' {
                    point.x = self.text_offset.get().x;
                    point.y += 16;
                } else {
                    if point.x + 8 <= w && point.y + 16 <= h {
                        renderer.char(point.x + rect.x, point.y + rect.y, c, self.fg);
                    }
                    point.x += 8;
                }
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        if self.visible.get(){
            match event {
                Event::Mouse { point, left_button, .. } => {
                    let mut click = false;

                    let rect = self.rect.get();
                    if rect.contains(point) {
                        if left_button {
                            if self.pressed.check_set(true) {
                                *redraw = true;
                            }
                        } else {
                            if self.pressed.check_set(false) {
                                click = true;
                                *redraw = true;
                            }
                        }
                    } else {
                        if !left_button {
                            if self.pressed.check_set(false) {
                                *redraw = true;
                            }
                        }
                    }

                    if click {
                        let click_point: Point = point - rect.point();
                        self.emit_click(click_point);
                    }
                }
                _ => (),
            }
        }

        focused
    }
    
    fn visible(&self, flag: bool){
        self.visible.set(flag);
    }    
}
