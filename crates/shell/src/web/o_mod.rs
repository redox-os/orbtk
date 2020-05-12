//! This module contains a platform specific implementation of the window shell.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
};

use stdweb::{
    js,
    traits::*,
    unstable::TryInto,
    web::{document, event, html_element::CanvasElement, window, CanvasRenderingContext2d},
};

use raw_window_handle::{web::WebHandle, HasRawWindowHandle, RawWindowHandle};

use lazy_static;

use crate::{prelude::*, render::*, utils::*, ShellRequest};

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn initialize() {
    set_panic_hook();
    stdweb::initialize();
}

fn get_mouse_button(button: event::MouseButton) -> MouseButton {
    match button {
        event::MouseButton::Wheel => MouseButton::Middle,
        event::MouseButton::Right => MouseButton::Right,
        _ => MouseButton::Left,
    }
}

fn get_key(code: &str, key: String) -> (Key, String) {
    let mut text = String::from("");

    let code = match code {
        "Backspace" => Key::Backspace,
        "Delete" => Key::Delete,
        "ControlLeft" | "ControlRight" => Key::Control,
        "ShiftLeft" => Key::ShiftL,
        "ShiftRight" => Key::ShiftR,
        "AltLeft" => Key::Alt,
        "AltRight" => Key::Alt,
        "ArrowUp" => Key::Up,
        "ArrowLeft" => Key::Left,
        "ArrowRight" => Key::Right,
        "ArrowDown" => Key::Down,
        "Escape" => Key::Escape,
        "Enter" => Key::Enter,
        "OSLeft" | "OSRight" => Key::Home,
        "CapsLock" => Key::CapsLock,
        _ => {
            text = key.clone();
            Key::from(key.chars().next().unwrap())
        }
    };

    (code, text)
}

/// Concrete implementation of the window shell.
pub struct Shell<A: 'static>
where
    A: WindowAdapter,
{
    render_context_2_d: RenderContext2D,
    pub mouse_move_events: Rc<RefCell<Vec<event::MouseMoveEvent>>>,
    pub mouse_up_events: Rc<RefCell<Vec<event::MouseUpEvent>>>,
    pub touch_start_events: Rc<RefCell<Vec<event::TouchStart>>>,
    pub touch_end_events: Rc<RefCell<Vec<event::TouchEnd>>>,
    pub touch_move_events: Rc<RefCell<Vec<event::TouchMove>>>,
    pub mouse_down_events: Rc<RefCell<Vec<event::MouseDownEvent>>>,
    pub scroll_events: Rc<RefCell<Vec<event::MouseWheelEvent>>>,
    pub key_up_events: Rc<RefCell<Vec<event::KeyUpEvent>>>,
    pub key_down_events: Rc<RefCell<Vec<event::KeyDownEvent>>>,
    pub resize_events: Rc<RefCell<Vec<event::ResizeEvent>>>,
    canvas: CanvasElement,
    pub old_canvas: Option<CanvasElement>,
    pub flip: bool,
    adapter: A,
    update: bool,
    running: bool,
    request_receiver: Receiver<ShellRequest>,
    request_sender: Sender<ShellRequest>,
}

unsafe impl<A> HasRawWindowHandle for Shell<A>
where
    A: WindowAdapter,
{
    fn raw_window_handle(&self) -> RawWindowHandle {
        let handle = WebHandle {
            id: 0,
            ..WebHandle::empty()
        };

        RawWindowHandle::Web(handle)
    }
}

impl<A> Shell<A>
where
    A: WindowAdapter,
{
    /// Gets if the shell is running.
    pub fn running(&self) -> bool {
        self.running
    }

    /// Gets a a new sender to send request to the window shell.
    pub fn request_sender(&self) -> Sender<ShellRequest> {
        self.request_sender.clone()
    }

    /// Sets running.
    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    /// Get if the shell should be updated.
    pub fn update(&self) -> bool {
        self.update
    }

    /// Sets update.
    pub fn set_update(&mut self, update: bool) {
        self.update = update;
    }

    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    /// Gets the render ctx 2D.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        &mut self.render_context_2_d
    }

    fn drain_events(&mut self) {
        while let Some(event) = self.mouse_move_events.borrow_mut().pop() {
            self.adapter
                .mouse(event.client_x() as f64, event.client_y() as f64);
        }

        while let Some(event) = self.mouse_down_events.borrow_mut().pop() {
            self.adapter.mouse_event(MouseEvent {
                x: event.client_x() as f64,
                y: event.client_y() as f64,
                button: get_mouse_button(event.button()),
                state: ButtonState::Down,
            });
        }

        while let Some(event) = self.mouse_up_events.borrow_mut().pop() {
            self.adapter.mouse_event(MouseEvent {
                x: event.client_x() as f64,
                y: event.client_y() as f64,
                button: get_mouse_button(event.button()),
                state: ButtonState::Up,
            });
        }

        while let Some(event) = self.scroll_events.borrow_mut().pop() {
            self.adapter.scroll(event.delta_x(), event.delta_y());
        }

        // todo tmp solution to map touch events to mouse vent
        while let Some(event) = self.touch_start_events.borrow_mut().pop() {
            self.adapter.mouse_event(MouseEvent {
                x: event.changed_touches()[0].client_x() as f64,
                y: event.changed_touches()[0].client_y() as f64,
                button: MouseButton::Left,
                state: ButtonState::Down,
            });
        }

        while let Some(event) = self.touch_end_events.borrow_mut().pop() {
            self.adapter.mouse_event(MouseEvent {
                x: event.changed_touches()[0].client_x() as f64,
                y: event.changed_touches()[0].client_y() as f64,
                button: MouseButton::Left,
                state: ButtonState::Up,
            });

            // self.mouse_blocked.set(false);
        }

        while let Some(event) = self.touch_move_events.borrow_mut().pop() {
            self.adapter.mouse(
                event.changed_touches()[0].client_x() as f64,
                event.changed_touches()[0].client_y() as f64,
            );
        }

        while let Some(event) = self.key_down_events.borrow_mut().pop() {
            let key = get_key(event.code().as_str(), event.key());

            self.adapter.key_event(KeyEvent {
                key: key.0,
                state: ButtonState::Down,
                text: key.1,
            });
        }

        while let Some(event) = self.key_up_events.borrow_mut().pop() {
            let key = get_key(event.code().as_str(), event.key());

            self.adapter.key_event(KeyEvent {
                key: key.0,
                state: ButtonState::Up,
                text: key.1,
            });
        }

        while let Some(_) = self.resize_events.borrow_mut().pop() {
            let window_size = (
                window().inner_width() as f64,
                window().inner_height() as f64,
            );

            let canvas: CanvasElement = document()
                .create_element("canvas")
                .unwrap()
                .try_into()
                .unwrap();

            canvas.set_width(window_size.0 as u32);
            canvas.set_height(window_size.1 as u32);

            js! {
                document.body.style.padding = 0;
                document.body.style.margin = 0;
                @{&canvas}.style.display = "block";
                @{&canvas}.style.margin = "0";
            }

            let device_pixel_ratio = window().device_pixel_ratio();
            let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

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

            self.render_context_2_d.set_canvas_render_context_2d(ctx);
            self.adapter.resize(window_size.0, window_size.1);
            self.old_canvas = Some(self.canvas.clone());
            self.canvas = canvas;
            self.flip = true;
        }

        // receive request
        let mut update = self.update();

        for request in self.request_receiver.try_iter() {
            if update {
                break;
            }

            match request {
                ShellRequest::Update => {
                    update = true;
                }
                _ => {}
            }
        }

        self.set_update(update);
    }

    pub fn flip(&mut self) {
        if !self.flip || !self.old_canvas.is_some() {
            return;
        }

        document()
            .body()
            .unwrap()
            .replace_child(&self.canvas, self.old_canvas.as_ref().unwrap())
            .expect("Could not open document");

        self.old_canvas = None;
        self.flip = false;
    }

    pub fn run(mut self) {
        window().request_animation_frame(move |_| {
            self.adapter.run(&mut self.render_context_2_d);
            self.set_update(false);
            self.flip();
            self.drain_events();
            self.run();
        });
    }
}

/// Constructs the window shell
pub struct ShellBuilder<A>
where
    A: WindowAdapter,
{
    title: String,

    borderless: bool,

    resizeable: bool,

    bounds: Rectangle,

    adapter: A,
}

impl<A> ShellBuilder<A>
where
    A: WindowAdapter,
{
    /// Create a new window builder with the given adapter.
    pub fn new(adapter: A) -> Self {
        ShellBuilder {
            adapter,
            title: String::default(),
            borderless: false,
            resizeable: false,
            bounds: Rectangle::default(),
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

    /// Does nothing on web.
    pub fn always_on_top(self, always_on_top: bool) -> Self {
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rectangle>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Builds the window shell.
    pub fn build(mut self) -> Shell<A> {
        // console_error_panic_hook::set_once();
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

        let render_context_2_d = RenderContext2D::from_context(canvas.get_context().unwrap());

        document().set_title(&self.title[..]);

        stdweb::event_loop();

        let (request_sender, request_receiver) = channel();

        Shell {
            render_context_2_d,
            adapter: self.adapter,
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
            flip: false,
            canvas,
            old_canvas: None,
            update: true,
            running: true,
            request_receiver,
            request_sender,
        }
    }
}

lazy_static! {
    pub static ref CONSOLE: Console = Console;
}

pub struct Console;

impl Console {
    pub fn time(&self, _name: impl Into<String>) {
        // js! {
        //     console.time(@{&name.into()})
        // }
    }

    pub fn time_end(&self, _name: impl Into<String>) {
        // js! {
        //     console.timeEnd(@{&name.into()})
        // }
    }

    pub fn log(&self, message: impl Into<String>) {
        js! {
            console.log(@{&message.into()});
        }
    }
}
