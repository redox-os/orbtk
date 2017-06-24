use orbclient::{Color, Renderer};
use rect::Rect;
use std::cell::{Cell, RefCell};
use cell::CheckSet;
use std::sync::Arc;
use point::Point;
use widgets::Widget;
use event::Event;
use traits::Click;
use traits::{Place, Container};
use theme::{ITEM_BORDER, ITEM_SELECTION};
use orbimage::Image;
use std::path::Path;


pub struct Checkbox {
    pub rect: Cell<Rect>,
    pub border_radius: Cell<u32>, // Determines how round the checkbox is
    pub bg_unchecked: Color, // Color of the background when unchecked
    pub bg_checked: Color, 
    pub fg_checked: Color,
    click_callback: RefCell<Option<Arc<Fn(&Checkbox, Point)>>>,
    pub checked: Cell<bool>, // True when the checkbox is checked
    pub pressed: Cell<bool>,
    pub check_size: f64, // A percentage of the larger rect
    pub images: RefCell<Option<(Image, Image)>>,
}

impl Checkbox {
    pub fn new() -> Arc<Self> {
        Arc::new(Checkbox {rect: Cell::new(Rect::default()),
                           border_radius: Cell::new(1),
                           bg_unchecked: ITEM_BORDER,
                           bg_checked: ITEM_BORDER,
                           fg_checked: ITEM_SELECTION,
                           click_callback: RefCell::new(None),
                           checked: Cell::new(false),
                           pressed: Cell::new(false),
                           check_size: 60.0,
                           images: RefCell::new(None),
        })
    }

    pub fn from_images(bg_image: Image, fg_image: Image) -> Arc<Self> {
        Arc::new(Checkbox {rect: Cell::new(Rect::new(0, 0,
                                                     bg_image.width(),
                                                     bg_image.height())),
                           border_radius: Cell::new(1),
                           bg_unchecked: ITEM_BORDER,
                           bg_checked: ITEM_BORDER,
                           fg_checked: ITEM_SELECTION,
                           click_callback: RefCell::new(None),
                           checked: Cell::new(false),
                           pressed: Cell::new(false),
                           check_size: 60.0,
                           images: RefCell::new(Some((bg_image, fg_image))),
        })
    }

    pub fn from_paths<P: AsRef<Path>, Q: AsRef<Path>>(bg_image: P, fg_image: Q) -> Result<Arc<Self>, String> {
        let image_bg = Image::from_path(bg_image)?;
        let image_fg = Image::from_path(fg_image)?;
        Ok(Checkbox::from_images(image_bg, image_fg))
    }

    pub fn flip_check(&self) -> &Self {
        self.checked.set(!self.checked.get());
        self
    }

    pub fn bg_get(&self) -> Color {
        if self.checked.get() {
            self.bg_checked
        } else {
            self.bg_unchecked
        }
    }

    pub fn filled_get(&self) -> bool {
        self.checked.get()
    }

}


impl Click for Checkbox {
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

impl Place for Checkbox {}

impl Widget for Checkbox {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        let rect = self.rect.get();
        let b_r = self.border_radius.get();
        if let Some(&(ref bg_image, ref fg_image)) = self.images.borrow().as_ref() {
            renderer.image(rect.x, rect.y,
                           bg_image.width(), bg_image.height(),
                           bg_image.data());
            if self.checked.get() {
                renderer.image(rect.x, rect.y,
                               fg_image.width(), fg_image.height(),
                               fg_image.data());
            }
        } else {
            renderer.rounded_rect(rect.x, rect.y, rect.width,
                                  rect.height, b_r, self.filled_get(), self.bg_get());
            if let Ok(rect_check) = rect.center(self.check_size) {
                if self.checked.get() {
                    renderer.rounded_rect(rect_check.x, rect_check.y,
                                          rect_check.width, rect_check.height,
                                          b_r, true, self.fg_checked);
                }
            } else if let Err(err) = rect.center(self.check_size) {
                println!("Error: {}", err);
            }

        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
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
                    self.flip_check();
                    let click_point: Point = point - rect.point();
                    self.emit_click(click_point);

                }
            }
            _ => (),
        }

        focused
    }

}
