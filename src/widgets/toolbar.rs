//toolbar widget is based mostly on image widget 

use orbclient::{Color, Renderer};
use orbimage;
use std::cell::{Cell, RefCell};
use cell::CloneCell;
use std::path::Path;
use std::sync::Arc;
use theme::{LABEL_BACKGROUND, LABEL_BORDER, LABEL_FOREGROUND};
use event::Event;
use point::Point;
use rect::Rect;
use traits::{Click, Place, Text}; //TODO create traits Tooltip , for now use Text
use widgets::Widget;
//use widgets::ToolbarWidget;

pub struct Toolbar {
    pub rect: Cell<Rect>,
    pub image: RefCell<orbimage::Image>,
    click_callback: RefCell<Option<Arc<Fn(&Toolbar, Point)>>>,
    pub visible: Cell<bool>,
    pub selected: Cell<bool>,
    pub tooltip: Cell<bool>,
    pub tooltip_text: CloneCell<String>,
    pub tooltip_offset: Cell<Point>,
    pub bg: Cell<Color>,
    pub fg: Cell<Color>,
    pub fg_border: Cell<Color>,
    pub border: Cell<bool>,
    pub border_radius: Cell<u32>,
 
}

impl Toolbar {
    pub fn new(width: u32, height: u32) -> Arc<Self> {
        Self::from_image(orbimage::Image::new(width, height))
    }

    pub fn from_color(width: u32, height: u32, color: Color) -> Arc<Self> {
        Self::from_image(orbimage::Image::from_color(width, height, color))
    }

    pub fn from_image(image: orbimage::Image) -> Arc<Self> {
        Arc::new(Toolbar {
            rect: Cell::new(Rect::new(0, 0, image.width(), image.height())),
            image: RefCell::new(image),
            click_callback: RefCell::new(None),
            visible: Cell::new(true),
            selected: Cell::new(false),
            tooltip: Cell::new(false),
            tooltip_text: CloneCell::new(String::new()),
            tooltip_offset: Cell::new(Point::default()),
            bg: Cell::new(LABEL_BACKGROUND),
            fg: Cell::new(LABEL_FOREGROUND),
            fg_border: Cell::new(LABEL_BORDER),
            border: Cell::new(false),
            border_radius: Cell::new(0),
        })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Arc<Self>, String> {
        Ok(Self::from_image(orbimage::Image::from_path(path)?))
    }
    
    pub fn selected(&self, flag: bool) {
        self.selected.set(flag);
    }
}

impl Click for Toolbar {
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

impl Place for Toolbar {}

// TODO create new traits Tooltip , for now workaround using Text
impl Text for Toolbar {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.tooltip_text.set(text.into());
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.tooltip_offset.set(Point::new(x, y));
        self
    }
}

impl Widget for Toolbar {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        if self.visible.get(){
            let rect = self.rect.get();
            let image = self.image.borrow();
            renderer.image(rect.x, rect.y, image.width(), image.height(), image.data());
            if self.selected.get(){
                renderer.rounded_rect(rect.x,rect.y, image.width()+1,image.height()+1,3,false,Color::rgb(0, 0, 0));
            }
        
    
    //draw tooltip
            
        if self.tooltip.get(){
            let b_r = self.border_radius.get();
            renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, true, self.bg.get());
            if self.border.get() {
                renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, false, self.fg_border.get());
            }

            let fg = self.fg.get();
            let text = self.tooltip_text.borrow();

            let mut point = self.tooltip_offset.get();
            for c in text.chars() {
                if c == '\n' {
                    point.x = self.tooltip_offset.get().x;
                    point.y += 16;
                } else {
                    if point.x + 8 <= rect.width as i32 && point.y + 16 <= rect.height as i32 {
                        renderer.char(point.x + rect.x, point.y + rect.y, c, fg);
                    }
                    point.x += 8;
                }
            }
         }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        if self.visible.get(){
            match event {
                Event::Mouse { point, left_button, right_button, .. } => {
                    let rect = self.rect.get();
                    if rect.contains(point) && left_button {
                        let click_point: Point = point - rect.point();
                        self.emit_click(click_point);
                        if self.selected.get() {
                            self.selected.set(false);
                        } else {
                            self.selected.set(true);
                        }
                        *redraw = true;
                    }
                    if rect.contains(point) && right_button {
                        self.selected.set(false);
                        *redraw = true;
                    }
                    if rect.contains(point) {
                        //TODO after 1 sec show up tooltip if point is unchanged
                        //println!("Mouse hovering toolbar at {} {} ",point.x,point.y);
                        
                        println!("Tooltip: {}",self.tooltip_text.get());
                        //self.tooltip.set(true);
                        //*redraw = true;
                    }
                }
                
                    
                _ => (),
            }
        }
        focused
    }
    
    fn visible(&self, flag: bool) {
        self.visible.set(flag);
    }
}


