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

fn adjust_ratio(canvas: &CanvasElement, device_pixel_ratio: f64, size: (f64, f64)) {
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
        canvas.set_width((size.0 as f64 * ratio) as u32);
        canvas.set_height((size.1 as f64 * ratio) as u32);

        js! {
            @{&canvas}.style.width = @{&size.0} + "px";
            @{&canvas}.style.height = @{&size.1} + "px";
        }

        context.scale(ratio, ratio);
    }
}

fn create_canvas(window_size: (f64, f64)) -> CanvasElement {
    let canvas: CanvasElement = document()
        .create_element("canvas")
        .unwrap()
        .try_into()
        .unwrap();

    adjust_ratio(&canvas, window().device_pixel_ratio(), window_size);

    js! {
        @{&canvas}.style.display = "block";
        @{&canvas}.style.margin = "0";
    };

    canvas
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
    device_pixel_ratio: f64,
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

        let mut resize = false;

        while let Some(_) = self.resize_events.borrow_mut().pop() {
            resize = true;
        }

        if resize {
            let window_size = (
                window().inner_width() as f64,
                window().inner_height() as f64,
            );

            self.render_context_2_d()
                .resize(window_size.0, window_size.1);

            self.adapter.resize(window_size.0, window_size.1);
        }
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
            // if self.window_shell.borrow_mut().device_pixel_ratio != window().device_pixel_ratio() {
            //     log(format!("ratio"));
            //     (*self.window_shell.borrow_mut()).device_pixel_ratio =
            //         window().device_pixel_ratio();
            //     adjust_ratio(
            //         &self.window_shell.borrow().canvas,
            //         window().device_pixel_ratio(),
            //         (
            //             window().inner_width() as f64,
            //             window().inner_height() as f64,
            //         ),
            //     );
            // }
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
        let window_size = (
            window().inner_width() as f64,
            window().inner_height() as f64,
        );

        let adapter = &mut self.adapter;
        adapter.resize(window_size.0, window_size.1);

        js! {
            document.body.style.padding = 0;
            document.body.style.margin = 0;
        }

        let first_canvas = create_canvas(window_size);
        document().body().unwrap().append_child(&first_canvas);

        let second_canvas = create_canvas(window_size);
        js! {
            @{&second_canvas}.style.visibility = "hidden";
        }
        document().body().unwrap().append_child(&second_canvas);

        // web event queues
        let mouse_move = Rc::new(RefCell::new(vec![]));
        let mouse_up = Rc::new(RefCell::new(vec![]));
        let mouse_down = Rc::new(RefCell::new(vec![]));
        let key_down = Rc::new(RefCell::new(vec![]));
        let key_up = Rc::new(RefCell::new(vec![]));
        let resize = Rc::new(RefCell::new(vec![]));

        let mouse_down_c = mouse_down.clone();
        window().add_event_listener(move |e: event::MouseDownEvent| {
            mouse_down_c.borrow_mut().push(e);
        });

        let mouse_up_c = mouse_up.clone();
        window().add_event_listener(move |e: event::MouseUpEvent| {
            mouse_up_c.borrow_mut().push(e);
        });

        let mouse_move_c = mouse_move.clone();
        window().add_event_listener(move |e: event::MouseMoveEvent| {
            mouse_move_c.borrow_mut().push(e);
        });

        let key_down_c = key_down.clone();
        document().add_event_listener(move |e: event::KeyDownEvent| {
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

        let render_context_2_d = RenderContext2D::new((first_canvas, second_canvas));

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
            device_pixel_ratio: window().device_pixel_ratio(),
        }
    }
}

pub fn log(message: String) {
    js! {
        console.log(@{&message});
    }
}
