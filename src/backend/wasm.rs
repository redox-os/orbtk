use std::cell::RefCell;
use std::rc::Rc;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::ResizeEvent;

use stdweb::web::html_element::CanvasElement;

// todo: workspace e.g. backend, strict spliting of world and backend

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

use {
    Backend, BackendRunner, EventContext, EventQueue, LayoutContext, Rect, RenderContext, Renderer,
    Selector, Theme, Tree, World,
};

pub struct CanvasRenderer {
    _canvas: CanvasElement,
    context: CanvasRenderingContext2d,
}

impl CanvasRenderer {
    pub fn new() -> CanvasRenderer {
        let canvas: CanvasElement = document()
            .query_selector("#canvas")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);

        window().add_event_listener(enclose!( (canvas) move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);

        }));

        CanvasRenderer {
            _canvas: canvas,
            context,
        }
    }
}

pub struct WasmBackend {
    running: bool,
    inner: CanvasRenderer,
    theme: Theme,
    event_queue: EventQueue,
}

impl WasmBackend {
    pub fn new(theme: Theme, inner: CanvasRenderer) -> WasmBackend {
        WasmBackend {
            inner,
            theme,
            event_queue: EventQueue::default(),
            running: false,
        }
    }
}

impl Renderer for CanvasRenderer {
    fn render(&mut self, theme: &Theme) {
        // render window background
        let col = theme.color("background", &"window".into());
        let blub = col.data;
        let mut _color = format!("#{:x}", blub);
        _color.remove(1);
        _color.remove(1);
        self.context.set_fill_style_color(&_color);
        self.context.fill_rect(0.0, 0.0, 420.0, 720.0);
    }

    fn render_rectangle(
        &mut self,
        _theme: &Theme,
        _bounds: &Rect,
        _selector: &Selector,
        _offset: (i32, i32),
    ) {
        // let b_r = theme.uint("border-radius", selector);

        let fill = _theme.color("background", _selector);

        // background
        if fill.data > 0 {
            let blub = fill.data;
            let mut _color = format!("#{:x}", blub);
            _color.remove(1);
            _color.remove(1);

           
            self.context.set_fill_style_color(&_color);
            self.context.fill_rect(
                (_offset.0 + _bounds.x) as f64,
                (_offset.1 + _bounds.x) as f64,
                _bounds.width as f64,
                _bounds.height as f64,
            );
        }

        // border
        let border_color = _theme.color("border-color", _selector);

        // background
        if border_color.data > 0 {
            let blub = border_color.data;
            let mut _color = format!("#{:x}", blub);
            _color.remove(1);
            _color.remove(1);

           self.context.set_line_width(1.0);
            self.context.set_stroke_style_color(&_color);
            self.context.stroke_rect(
                (_offset.0 + _bounds.x) as f64,
                (_offset.1 + _bounds.x) as f64,
                _bounds.width as f64,
                _bounds.height as f64,
            );
        }

        // self.rounded_rect(
        //     bounds.x + offset.0,
        //     bounds.y + offset.1,
        //     bounds.width,
        //     bounds.height,
        //     b_r,
        //     true,
        //     fill,
        // );

        // if theme.uint("border-width", selector) > 0 {
        //     let border_color = theme.color("border-color", selector);

        //     self.rounded_rect(
        //         bounds.x + offset.0,
        //         bounds.y + offset.1,
        //         bounds.width,
        //         bounds.height,
        //         b_r,
        //         false,
        //         border_color,
        //     );
        // }
    }

    fn render_text(
        &mut self,
        _theme: &Theme,
        _text: &str,
        _bounds: &Rect,
        _selector: &Selector,
        _offset: (i32, i32),
    ) {
        let fill = _theme.color("color", _selector);
        if fill.data > 0 {
            let blub = fill.data;
            let mut _color = format!("#{:x}", blub);
            _color.remove(1);
            _color.remove(1);

           

            self.context.set_fill_style_color(&_color);
        }

        self.context.fill_text(
                _text,
                (_offset.0 + _bounds.x) as f64 + (_bounds.width as f64 / 4.0),
                (_offset.1 + _bounds.y) as f64 + (_bounds.height as f64 / 1.5),
                Some(_bounds.width as f64)
            );
    }
}

impl Backend for WasmBackend {
    fn drain_events(&mut self) {}

    fn size(&self) -> (u32, u32) {
        (420, 720)
    }

    fn bounds(&mut self, _bounds: &Rect) {}

    fn render_context(&mut self) -> RenderContext {
        RenderContext {
            renderer: &mut self.inner,
            theme: &self.theme,
        }
    }

    fn layout_context(&mut self) -> LayoutContext {
        LayoutContext {
            window_size: self.size(),
            theme: &self.theme,
        }
    }

    fn event_context(&mut self) -> EventContext {
        EventContext {
            event_queue: &mut self.event_queue,
        }
    }
}

pub struct WasmBackendRunner {
    pub world: Option<World<Tree>>,
    pub backend: Rc<RefCell<WasmBackend>>,
}

impl BackendRunner for WasmBackendRunner {
    fn world(&mut self, world: World<Tree>) {
        self.world = Some(world);
    }
    fn run(&mut self) {
        self.backend.borrow_mut().running = true;

        stdweb::initialize();

        stdweb::event_loop();

        if let Some(world) = &mut self.world {
            world.run();
        }

        // loop {
        //      context.fill_rect(0.0, 0.0, 400.0, 400.0);
        //     if let Some(world) = &mut self.world {
        //         world.run();
        //     }
        // }
    }
}
