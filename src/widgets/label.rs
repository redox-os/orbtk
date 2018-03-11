use std::cell::{Cell, RefCell};
use std::sync::Arc;

use cell::{CloneCell, CheckSet};
use events::Event;
use point::Point;
use rect::Rect;
use thickness::Thickness;
use theme::Selector;
use traits::{Click, Place, Text, Style};
use widgets::{Widget, VerticalPlacement, HorizontalPlacement};
use primitives::{TextWidget, Rectangle};

const LABEL_SELECTOR: &str = "label";

pub struct Label {
    pub rect: Cell<Rect>,
    local_position: Cell<Point>,
    vertical_placement: Cell<VerticalPlacement>,
    horizontal_placement: Cell<HorizontalPlacement>,
    margin: Cell<Thickness>,
    children: RefCell<Vec<Arc<Widget>>>,
    pub selector: CloneCell<Selector>,
    pub border: Cell<bool>,
    pub border_radius: Cell<u32>,
    pub text: CloneCell<String>,
    pub text_offset: Cell<Point>,
    click_callback: RefCell<Option<Arc<Fn(&Label, Point)>>>,
    pressed: Cell<bool>,
}

impl Label {
    pub fn new() -> Arc<Self> {
        let selector = CloneCell::new(Selector::new(Some(LABEL_SELECTOR)));
        let inner_text = CloneCell::new(String::new());

        let text_widget = TextWidget::new();
        text_widget.inner_text().bind(&inner_text);
        text_widget.selector().bind(&selector);
        text_widget.placement(VerticalPlacement::Center, HorizontalPlacement::Center);

        let background = Rectangle::new();
        background.selector().bind(&selector);
        background.placement(VerticalPlacement::Stretch, HorizontalPlacement::Stretch);
        background.add(text_widget);

        Arc::new(Label {
            rect: Cell::new(Rect::default()),
            local_position: Cell::new(Point::new(0, 0)),
            vertical_placement: Cell::new(VerticalPlacement::Absolute),
            horizontal_placement: Cell::new(HorizontalPlacement::Absolute),
            margin: Cell::new(Thickness::default()),
            children: RefCell::new(vec![background]),
            selector,
            border: Cell::new(false),
            border_radius: Cell::new(0),
            text: inner_text,
            text_offset: Cell::new(Point::default()),
            click_callback: RefCell::new(None),
            pressed: Cell::new(false),
        })
    }

    fn adjust_size(&self) {
        self.size(
            self.text.get().len() as u32 * 8 + 2 * self.text_offset.get().x as u32,
            16 + 2 * self.text_offset.get().y as u32,
        );
    }
}

impl Click for Label {
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

impl Place for Label {}

impl Text for Label {
    fn text<S: Into<String>>(&self, text: S) -> &Self {
        self.text.set(text.into());
        self.adjust_size();
        self
    }

    fn text_offset(&self, x: i32, y: i32) -> &Self {
        self.text_offset.set(Point::new(x, y));
        self.adjust_size();
        self
    }
}

impl Style for Label {
    fn selector(&self) -> &CloneCell<Selector> {
        &self.selector
    }
}

impl Widget for Label {
    fn name(&self) -> &str {
        "Label"
    }

    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn local_position(&self) -> &Cell<Point> {
        &self.local_position
    }

    fn vertical_placement(&self) -> &Cell<VerticalPlacement> {
        &self.vertical_placement
    }

    fn horizontal_placement(&self) -> &Cell<HorizontalPlacement> {
        &self.horizontal_placement
    }

    fn margin(&self) -> &Cell<Thickness> {
        &self.margin
    }

    // fn event(&self, event: Event, redraw: &mut bool) -> bool {
    //     match event {
    //         Event::Mouse { point, left_button, .. } => {
    //             let mut click = false;

    //             let rect = self.rect.get();
    //             if rect.contains(point) {
    //                 if left_button {
    //                     if self.pressed.check_set(true) {
    //                         *redraw = true;
    //                     }
    //                 } else {
    //                     if self.pressed.check_set(false) {
    //                         click = true;
    //                         *redraw = true;
    //                     }
    //                 }
    //             } else {
    //                 if !left_button {
    //                     if self.pressed.check_set(false) {
    //                         *redraw = true;
    //                     }
    //                 }
    //             }

    //             if click {
    //                 let click_point: Point = point - rect.point();
    //                 self.emit_click(click_point);
    //             }
    //         }
    //         _ => (),
    //     }

    //     focused
    // }

    fn children(&self) -> &RefCell<Vec<Arc<Widget>>> {
        &self.children
    }
}
