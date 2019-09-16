//! This module contains a platform specific implementation of the window shell.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use stdweb::{
    _js_impl, js,
    traits::*,
    unstable::TryInto,
    web::{document, event, html_element::CanvasElement, window, CanvasRenderingContext2d},
};

use crate::{prelude::*, render::*, utils::*};

pub fn initialize() {
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
        "ControlLeft" => Key::Control,
        "ShiftLeft" => Key::ShiftL,
        "ShiftRight" => Key::ShiftR,
        "AltLeft" => Key::Alt,
        "ArrowUp" => Key::Up,
        "ArrowLeft" => Key::Left,
        "ArrowRight" => Key::Right,
        "ArrowDown" => Key::Down,
        _ => {
            text = key.clone();
            Key::from(key.chars().next().unwrap())
        }
    };

    (code, text)
}

/// Concrete implementation of the window shell.
pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    render_context_2_d: RenderContext2D,
    pub mouse_move_events: Rc<RefCell<Vec<event::MouseMoveEvent>>>,
    pub mouse_up_events: Rc<RefCell<Vec<event::MouseUpEvent>>>,
    pub mouse_down_events: Rc<RefCell<Vec<event::MouseDownEvent>>>,
    pub key_up_events: Rc<RefCell<Vec<event::KeyUpEvent>>>,
    pub key_down_events: Rc<RefCell<Vec<event::KeyDownEvent>>>,
    pub resize_events: Rc<RefCell<Vec<event::ResizeEvent>>>,
    canvas: CanvasElement,
    pub old_canvas: Option<CanvasElement>,
    pub flip: bool,
    adapter: A,
}

impl<A> WindowShell<A>
where
    A: WindowAdapter,
{
    /// Gets the shell adapter.
    pub fn adapter(&mut self) -> &mut A {
        &mut self.adapter
    }

    /// Gets the render context 2D.
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

        while let Some(event) = self.key_down_events.borrow_mut().pop() {
            let key = get_key(event.code().as_str(), event.key());

            self.adapter.key_event(KeyEvent {
                key: key.0,
                state: ButtonState::Down,
                text: key.1,
            });
        }

        while let Some(event) = self.resize_events.borrow_mut().pop() {
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
            let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

            let backing_store_ratio = js! {
                var context = @{&context};
                 return context.webkitBackingStorePixelRatio ||
                     context.mozBackingStorePixelRatio ||
                     context.msBackingStorePixelRatio ||
                     context.oBackingStorePixelRatio ||
                     context.backingStorePixelRatio || 1;
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

                context.scale(ratio, ratio);
            }

            self.render_context_2_d
                .set_canvas_render_context_2d(context);
            self.adapter
                .resize(window_size.0, window_size.1);
            self.old_canvas = Some(self.canvas.clone());
            self.canvas = canvas;
            self.flip = true;
        }
    }

    pub fn flip(&mut self) {
        if !self.flip || !self.old_canvas.is_some() {
            return;
        }

        log(format!(
            "CSize: {},{}",
            self.canvas.width(),
            self.canvas.height()
        ));

        document()
            .body()
            .unwrap()
            .replace_child(&self.canvas, self.old_canvas.as_ref().unwrap());

        self.old_canvas = None;
        self.flip = false;
    }
}

/// Implementation of the OrbClient based shell runner.
pub struct ShellRunner<A>
where
    A: WindowAdapter + 'static,
{
    pub window_shell: Rc<RefCell<WindowShell<A>>>,
    pub update: Rc<Cell<bool>>,
    pub running: Rc<Cell<bool>>,
    pub updater: Box<dyn Updater>,
}

impl<A> ShellRunner<A>
where
    A: WindowAdapter,
{
    pub fn run(mut self) {
        window().request_animation_frame(move |_| {
            self.updater.update();
            self.update.set(false);
            self.window_shell.borrow_mut().drain_events();
            self.run();
        });
    }
}

/// Constructs the window shell
pub struct WindowBuilder<A>
where
    A: WindowAdapter,
{
    title: String,

    resizeable: bool,

    bounds: Rect,

    adapter: A,
}

impl<A> WindowBuilder<A>
where
    A: WindowAdapter,
{
    /// Create a new window builder with the given adapter.
    pub fn new(adapter: A) -> Self {
        WindowBuilder {
            adapter,
            title: String::default(),
            resizeable: false,
            bounds: Rect::default(),
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets resizeable.
    pub fn resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = resizeable;
        self
    }

    /// Sets the bounds.
    pub fn bounds(mut self, bounds: impl Into<Rect>) -> Self {
        self.bounds = bounds.into();
        self
    }

    /// Builds the window shell.
    pub fn build(mut self) -> WindowShell<A> {
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
        let mouse_down = Rc::new(RefCell::new(vec![]));
        let key_down = Rc::new(RefCell::new(vec![]));
        let key_up = Rc::new(RefCell::new(vec![]));
        let resize = Rc::new(RefCell::new(vec![]));

        let mouse_down_c = mouse_down.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::MouseDownEvent| {
                mouse_down_c.borrow_mut().push(e);
            });

        let mouse_up_c = mouse_up.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::MouseUpEvent| {
                mouse_up_c.borrow_mut().push(e);
            });

        let mouse_move_c = mouse_move.clone();
        document()
            .body()
            .unwrap()
            .add_event_listener(move |e: event::MouseMoveEvent| {
                mouse_move_c.borrow_mut().push(e);
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
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let device_pixel_ratio = window().device_pixel_ratio();

        let backing_store_ratio = js! {
            var context = @{&context};
             return context.webkitBackingStorePixelRatio ||
                 context.mozBackingStorePixelRatio ||
                 context.msBackingStorePixelRatio ||
                 context.oBackingStorePixelRatio ||
                 context.backingStorePixelRatio || 1;
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

            context.scale(ratio, ratio);
        }

        let render_context_2_d = RenderContext2D::new(canvas.get_context().unwrap());

        document().set_title(&self.title[..]);

        stdweb::event_loop();

        WindowShell {
            render_context_2_d,
            adapter: self.adapter,
            mouse_move_events: mouse_move,
            mouse_up_events: mouse_up,
            mouse_down_events: mouse_down,
            key_down_events: key_down,
            key_up_events: key_up,
            resize_events: resize,
            flip: false,
            canvas,
            old_canvas: None,
        }
    }
}

pub fn log(message: impl Into<String>) {
    js! {
        console.log(@{&message.into()});
    }
}
