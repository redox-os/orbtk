use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    sync::mpsc,
    // time::Duration,
};

use stdweb::{
    js,
    traits::*,
    unstable::TryInto,
    web::{document, event, html_element::CanvasElement, window, CanvasRenderingContext2d},
};

use super::{EventState, Shell, Window};
use crate::{
    render::RenderContext2D, utils::Rectangle, window_adapter::WindowAdapter, WindowRequest,
    WindowSettings,
};

/// The `WindowBuilder` is used to construct a window shell for the web backend.
pub struct WindowBuilder<'a, A: 'static>
where
    A: WindowAdapter,
{
    shell: &'a mut Shell<A>,
    adapter: A,
    title: String,
    resizeable: bool,
    always_on_top: bool,
    borderless: bool,
    fonts: HashMap<String, &'static [u8]>,
    bounds: Rectangle,
    request_receiver: Option<mpsc::Receiver<WindowRequest>>,
}

impl<'a, A> WindowBuilder<'a, A>
where
    A: WindowAdapter,
{
    /// Creates a new window builder.
    pub fn new(shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            shell,
            adapter,
            title: String::default(),
            resizeable: false,
            always_on_top: false,
            borderless: false,
            fonts: HashMap::new(),
            bounds: Rectangle::new((0.0, 0.0), (100.0, 75.0)),
            request_receiver: None,
        }
    }

    /// Creates the window builder from a settings object.
    pub fn from_settings(settings: WindowSettings, shell: &'a mut Shell<A>, adapter: A) -> Self {
        WindowBuilder {
            shell,
            adapter,
            title: settings.title,
            resizeable: settings.resizeable,
            always_on_top: settings.always_on_top,
            borderless: settings.borderless,
            fonts: settings.fonts,
            bounds: Rectangle::new(settings.position, (settings.size.0, settings.size.1)),
            request_receiver: None,
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Sets always_on_top.
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Registers a new font with family key.
    pub fn font(mut self, family: impl Into<String>, font_file: &'static [u8]) -> Self {
        self.fonts.insert(family.into(), font_file);
        self
    }

    /// Register a window request receiver to communicate with the window shell from outside.
    pub fn request_receiver(mut self, request_receiver: mpsc::Receiver<WindowRequest>) -> Self {
        self.request_receiver = Some(request_receiver);
        self
    }

    /// Builds the window shell and add it to the application `Shell`.
    pub fn build(mut self) {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();

        let window_size = (
            window().inner_width() as f64,
            window().inner_height() as f64,
        );

        canvas.set_width(window_size.0 as u32);
        canvas.set_height(window_size.1 as u32);

        let adapter = &mut self.adapter;
        adapter.resize(window_size.0, window_size.1);

        js! {
            document.body.style.padding = 0;
            document.body.style.margin = 0;
            @{&canvas}.style.display = "block";
            @{&canvas}.style.margin = "0";
        }

        // web event queues
        let mouse_move = Rc::new(RefCell::new(vec![]));
        let mouse_up = Rc::new(RefCell::new(vec![]));
        let touch_start = Rc::new(RefCell::new(vec![]));
        let touch_end = Rc::new(RefCell::new(vec![]));
        let touch_move = Rc::new(RefCell::new(vec![]));
        let mouse_down = Rc::new(RefCell::new(vec![]));
        let scroll = Rc::new(RefCell::new(vec![]));
        let key_down = Rc::new(RefCell::new(vec![]));
        let key_up = Rc::new(RefCell::new(vec![]));
        let resize = Rc::new(RefCell::new(vec![]));
        let mouse_blocked = Rc::new(Cell::new(false));

        let mouse_down_c = mouse_down.clone();
        let mouse_blocked_c = mouse_blocked.clone();

        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::MouseDownEvent| {
                if !mouse_blocked_c.get() {
                    mouse_down_c.borrow_mut().push(e);
                }
            });

        let mouse_blocked_c = mouse_blocked.clone();
        let mouse_up_c = mouse_up.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::MouseUpEvent| {
                if !mouse_blocked_c.get() {
                    mouse_up_c.borrow_mut().push(e);
                }
            });

        let touch_start_c = touch_start.clone();
        let mouse_blocked_c = mouse_blocked.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::TouchStart| {
                mouse_blocked_c.set(true);
                touch_start_c.borrow_mut().push(e);
            });

        let touch_end_c = touch_end.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::TouchEnd| {
                touch_end_c.borrow_mut().push(e);
            });

        let touch_move_c = touch_move.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::TouchMove| {
                touch_move_c.borrow_mut().push(e);
            });

        let mouse_move_c = mouse_move.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::MouseMoveEvent| {
                mouse_move_c.borrow_mut().push(e);
            });

        let scroll_c = scroll.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::MouseWheelEvent| {
                scroll_c.borrow_mut().push(e);
            });

        let key_down_c = key_down.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::KeyDownEvent| {
                e.prevent_default();
                key_down_c.borrow_mut().push(e);
            });

        let key_up_c = key_up.clone();
        document().add_event_listener(move |e: event::KeyUpEvent| {
            e.prevent_default();
            key_up_c.borrow_mut().push(e);
        });

        let resize_c = resize.clone();
        window().add_event_listener(move |e: event::ResizeEvent| {
            e.prevent_default();
            resize_c.borrow_mut().push(e);
        });

        document().body().unwrap().append_child(&canvas);
        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let device_pixel_ratio = window().device_pixel_ratio();

        let backing_store_ratio = js! {
            var ctx = @{&ctx};
             return ctx.webkitBackingStorePixelRatio ||
                 ctx.mozBackingStorePixelRatio ||
                 ctx.msBackingStorePixelRatio ||
                 ctx.oBackingStorePixelRatio ||
                 ctx.backingStorePixelRatio || 1;
        };

        let ratio: f64 = js! {
            return @{&device_pixel_ratio} / @{&backing_store_ratio};
        }
        .try_into()
        .unwrap();

        if device_pixel_ratio != backing_store_ratio {
            let old_width = canvas.width();
            let old_height = canvas.height();
            canvas.set_width((old_width as f64 * ratio) as u32);
            canvas.set_height((old_height as f64 * ratio) as u32);

            js! {
                @{&canvas}.style.width = @{&old_width} + "px";
                @{&canvas}.style.height = @{&old_height} + "px";
            }

            ctx.scale(ratio, ratio);
        }

        let render_context = RenderContext2D::from_context(canvas.get_context().unwrap());

        document().set_title(self.title.as_str());

        stdweb::event_loop();

        self.shell.window_shells.push(Window::new(
            self.adapter,
            render_context,
            self.request_receiver,
            EventState {
                mouse_move_events: mouse_move,
                mouse_up_events: mouse_up,
                touch_start_events: touch_start,
                touch_end_events: touch_end,
                touch_move_events: touch_move,
                mouse_down_events: mouse_down,
                scroll_events: scroll,
                key_down_events: key_down,
                key_up_events: key_up,
                resize_events: resize,
            },
            canvas,
        ));
    }
}
