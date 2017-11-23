use orbclient::{Color, Renderer};
use orbimage;
use std::cell::{Cell, RefCell};
use std::path::Path;
use std::sync::Arc;

use event::Event;
use point::Point;
use rect::Rect;
use traits::{Click, Place};
use widgets::Widget;

pub struct Image {
    pub rect: Cell<Rect>,
    pub image: RefCell<orbimage::Image>,
    click_callback: RefCell<Option<Arc<Fn(&Image, Point)>>>,
    pub visible: Cell<bool>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Arc<Self> {
        Self::from_image(orbimage::Image::new(width, height))
    }

    pub fn from_color(width: u32, height: u32, color: Color) -> Arc<Self> {
        Self::from_image(orbimage::Image::from_color(width, height, color))
    }

    pub fn from_image(image: orbimage::Image) -> Arc<Self> {
        Arc::new(Image {
            rect: Cell::new(Rect::new(0, 0, image.width(), image.height())),
            image: RefCell::new(image),
            click_callback: RefCell::new(None),
            visible: Cell::new(true),
        })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Arc<Self>, String> {
        Ok(Self::from_image(orbimage::Image::from_path(path)?))
    }
}

impl Click for Image {
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

impl Place for Image {}

impl Widget for Image {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        if self.visible.get(){
            let rect = self.rect.get();
            let image = self.image.borrow();
            renderer.image(rect.x, rect.y, image.width(), image.height(), image.data());
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        if self.visible.get(){
            match event {
                Event::Mouse { point, left_button, .. } => {
                    let rect = self.rect.get();
                    if rect.contains(point) && left_button {
                        let click_point: Point = point - rect.point();
                        self.emit_click(click_point);
                        *redraw = true;
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
    
    fn name(&self) -> Option<&'static str> {
        Some("Image")
    }
}
