//toolbar widget is based mostly on image widget 

use orbclient::{Color, Renderer};
use orbimage;
use std::cell::{Cell, RefCell};
use cell::CloneCell;
use std::path::Path;
use std::sync::Arc;

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
    pub tooltip: CloneCell<String>,
    pub tooltip_offset: Cell<Point>,
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
            tooltip: CloneCell::new(String::new()),
            tooltip_offset: Cell::new(Point::default()),
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
        self.tooltip.set(text.into());
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
                        //TODO after 1 sec show up tooltip
                        //println!("Mouse hovering toolbar at {} {}",point.x,point.y);
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

/*
impl ToolbarWidget for Toolbar {
    fn selected(&self, flag: bool) {
        self.selected.set(flag);
    }
}

        
*/
