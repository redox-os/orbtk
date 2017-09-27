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
use window::Window;

#[allow(unused_imports)]
use std::time::{Duration, Instant};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Toolbar {
    //parent : RefCell<*mut Window>,
    pub items : RefCell<Vec<Arc<ToolbarIcon>>>,
    pub visible: Cell<bool>,
    pub selected: Cell<bool>,
    //pub rect: Cell<Rect>,
    
}
#[allow(dead_code)]
impl Toolbar {
    pub fn new() -> Self {
        Toolbar{
            items : RefCell::new(Vec::new()),
            visible: Cell::new(true),
            selected : Cell::new(true)
        }
    }
    
    pub fn add(&self, toolbar_icon: &Arc<ToolbarIcon>, window:*mut Window ) -> usize {
        let mut items = self.items.borrow_mut();
        let id = items.len();
        items.push(toolbar_icon.clone());
        //add also to parent window
        unsafe{(&mut *window).add(&toolbar_icon.clone());}
        id
    }
    
    pub fn visible (&self, v: bool) {
        //set visibility for all items of toolbar 
        let items = self.items.borrow_mut();
        for i in 0..items.len(){
            if let Some(tolbar_icon) = items.get(i) {
                tolbar_icon.visible.set(v);
            }
        }
    }
    
    pub fn toggle (&self) {
        //deselect all items from toolbar 
        let items = self.items.borrow_mut();
        for i in 0..items.len(){
            if let Some(toolbar_icon) = items.get(i) {
                toolbar_icon.selected(false);
            }
        }
    }    
}


pub struct ToolbarIcon {
    pub rect: Cell<Rect>,
    pub image: RefCell<orbimage::Image>,
    click_callback: RefCell<Option<Arc<Fn(&ToolbarIcon, Point)>>>,
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
    tooltip_time : Cell<Option<Instant>>,
    
 
}

impl ToolbarIcon {
    pub fn new(width: u32, height: u32) -> Arc<Self> {
        Self::from_image(orbimage::Image::new(width, height))
    }

    pub fn from_color(width: u32, height: u32, color: Color) -> Arc<Self> {
        Self::from_image(orbimage::Image::from_color(width, height, color))
    }

    pub fn from_image(image: orbimage::Image) -> Arc<Self> {
        Arc::new(ToolbarIcon {
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
            border: Cell::new(true),
            border_radius: Cell::new(0),
            tooltip_time : Cell::new(None),
            
        })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Arc<Self>, String> {
        Ok(Self::from_image(orbimage::Image::from_path(path)?))
    }
    
    pub fn selected(&self, flag: bool) {
        self.selected.set(flag);
    }
}

impl Click for ToolbarIcon {
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

impl Place for ToolbarIcon {}

// TODO create new traits Tooltip , for now workaround using Text
impl Text for ToolbarIcon {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.tooltip_text.set(text.into());
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.tooltip_offset.set(Point::new(x, y));
        self
    }
}

impl Widget for ToolbarIcon {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        if self.visible.get(){
            let mut rect = self.rect.get();
            let image = self.image.borrow();
            renderer.image(rect.x, rect.y, image.width(), image.height(), image.data());
            if self.selected.get(){
                renderer.rounded_rect(rect.x,rect.y, image.width()+1,image.height()+1,3,false,Color::rgb(0, 0, 0));
            }
        
    
    //draw tooltip
            
        if self.tooltip.get(){
            let text = self.tooltip_text.borrow();
            rect = Rect::new(rect.x, rect.y+rect.height as i32, 8* text.len() as u32, 16); 
            let b_r = self.border_radius.get();
            let bg = Color::rgb(255,255,100); //yellow tooltip background
            renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, true, bg);
            if self.border.get() {
                renderer.rounded_rect(rect.x, rect.y, rect.width, rect.height, b_r, false, self.fg_border.get());
            }

            let fg = self.fg.get();
            

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
                        //FIXME after 2 sec shows up tooltip but only if mouse is moved 
                            match self.tooltip_time.get() {
                                Some(time) => {
                                                if !self.tooltip.get(){
                                                    if (Instant::now()-time) > Duration::new(2,0){
                                                        //println!("Tooltip: {} time:{:?}",self.tooltip_text.get(),time);
                                                        self.tooltip.set(true);
                                                        *redraw = true;
                                                        }
                                                    }
                                                },
                                None       => self.tooltip_time.set(Some(Instant::now())),
                            }
                        //self.tooltip.set(true);
                        //*redraw = true;
                    }else{
                        self.tooltip_time.set(None);
                        self.tooltip.set(false);
                        *redraw = true;
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


