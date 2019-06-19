//! This module contains a platform specific implementation of the window shell.

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    sync::Arc,
};

use orbgl_api::{Canvas, Font};
use orbgl_web::prelude::*;
use stdweb::{
    _js_impl, js,
    traits::*,
    unstable::TryInto,
    web::{
        self, document, event,
        html_element::{CanvasElement, ImageElement},
        window, CanvasRenderingContext2d, FillRule,
    },
};

use crate::{obsolete, prelude::*, utils::*, render::*};

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

fn get_key(code: &str, key: char) -> Key {
    match code {
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
        _ => match key {
            '\n' => Key::Enter,
            _ => Key::from(key),
        },
    }
}

/// Concrete implementation of the window shell.
pub struct WindowShell<A>
where
    A: WindowAdapter,
{
    render_context_2D: RenderContext2D,
    pub inner: WebRenderer,
    pub canvas: Canvas,
    pub mouse_move_events: Rc<RefCell<Vec<event::MouseMoveEvent>>>,
    pub mouse_up_events: Rc<RefCell<Vec<event::MouseUpEvent>>>,
    pub mouse_down_events: Rc<RefCell<Vec<event::MouseDownEvent>>>,
    pub key_up_events: Rc<RefCell<Vec<event::KeyUpEvent>>>,
    pub key_down_events: Rc<RefCell<Vec<event::KeyDownEvent>>>,
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

    pub fn render_context_2D(&mut self) -> &mut RenderContext2D {
        &mut self.render_context_2D
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
            self.adapter.key_event(KeyEvent {
                key: get_key(event.code().as_str(), event.key().remove(0)),
                state: ButtonState::Down,
            });
        }

        while let Some(event) = self.key_up_events.borrow_mut().pop() {
            self.adapter.key_event(KeyEvent {
                key: get_key(event.code().as_str(), event.key().remove(0)),
                state: ButtonState::Up,
            });
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
    pub updater: Box<Updater>,
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
    pub fn build(self) -> WindowShell<A> {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();

        canvas.set_width(self.bounds.width as u32);
        canvas.set_height(self.bounds.height as u32);

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

        let mouse_down_c = mouse_down.clone();
        canvas.add_event_listener(move |e: event::MouseDownEvent| {
            mouse_down_c.borrow_mut().push(e);
        });

        let mouse_up_c = mouse_up.clone();
        canvas.add_event_listener(move |e: event::MouseUpEvent| {
            mouse_up_c.borrow_mut().push(e);
        });

        let mouse_move_c = mouse_move.clone();
        canvas.add_event_listener(move |e: event::MouseMoveEvent| {
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

        document().body().unwrap().append_child(&canvas);
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let devicePixelRatio = window().device_pixel_ratio();

        let backingStoreRatio = js! {
            var context = @{&context};
             return context.webkitBackingStorePixelRatio ||
                 context.mozBackingStorePixelRatio ||
                 context.msBackingStorePixelRatio ||
                 context.oBackingStorePixelRatio ||
                 context.backingStorePixelRatio || 1;
        };

        let ratio: f64 = js! {
            return @{&devicePixelRatio} / @{&backingStoreRatio};
        }
        .try_into()
        .unwrap();

        if devicePixelRatio != backingStoreRatio {
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

        let surface = WebSurface::new(self.bounds.width as u32, self.bounds.height as u32, context);
        let render_engine = WebRenderEngine::new(surface);
        let render_context_2D = RenderContext2D::new(canvas.get_context().unwrap());
        let mut canvas = Canvas::new(render_engine.clone());

        document().set_title(&self.title[..]);

        stdweb::event_loop();

        WindowShell {
            render_context_2D,
            inner: WebRenderer {},
            canvas,
            adapter: self.adapter,
            mouse_move_events: mouse_move,
            mouse_up_events: mouse_up,
            mouse_down_events: mouse_down,
            key_down_events: key_down,
            key_up_events: key_up,
        }
    }
}

pub fn log(message: String) {
    js! {
        console.log(@{&message});
    }
}

// --- obsolete will be removed after OrbGL supports text rendering ---

pub struct WebFontMeasure;

impl FontMeasure for WebFontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32) {
        let canvas: CanvasElement = document()
            .query_selector("canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        context.set_font(&format!("{}px {}", font_size, font.family));

        (
            context.measure_text(text).unwrap().get_width() as u32,
            font_size,
        )
    }
}

lazy_static! {
    pub static ref FONT_MEASURE: Arc<WebFontMeasure> = { Arc::new(WebFontMeasure) };
}

pub struct WebRenderer {}

impl obsolete::Renderer for WebRenderer {
    fn render_text(
        &mut self,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        global_position: &Point,
        font_size: u32,
        color: Color,
        font: &Font,
    ) {
        if color.r() == 0 && color.g() == 0 && color.b() == 0 && color.a() == 0 {
            return;
        }
        let canvas: CanvasElement = document()
            .query_selector("canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        context.save();
        context.begin_path();
        context.rect(
            global_position.x,
            global_position.y,
            parent_bounds.width,
            parent_bounds.height,
        );
        context.clip(FillRule::EvenOdd);
        context.set_font(&format!("{}px {}", font_size, font.family));
        context.set_fill_style_color(&color.to_string());
        context.set_text_baseline(web::TextBaseline::Top);
        context.fill_text(
            text,
            global_position.x + bounds.x,
            global_position.y + bounds.y,
            None,
        );
        context.close_path();
        context.restore();
    }
}

// --- obsolete will be removed after OrbGL supports text rendering ---
