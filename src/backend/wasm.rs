use std::sync::Arc;
use std::sync::{mpsc::Receiver, mpsc::Sender};

use {Backend, EventManager, Rect, RenderContainer, Theme};

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::{MouseMoveEvent, ResizeEvent};

use stdweb::web::html_element::CanvasElement;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub struct WasmBackend {
    _theme: Arc<Theme>,
}

impl WasmBackend {
    pub fn new(theme: Arc<Theme>) -> WasmBackend {
        WasmBackend {
            _theme: theme
        }
    }
}

impl Backend for WasmBackend {
    fn drain_events(&mut self) {}

    fn size(&self) -> (u32, u32) {
        (430, 720)
    }

    fn bounds(&mut self, _bounds: &Rect) {}

    fn run(&mut self) {
        stdweb::initialize();

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

        canvas.add_event_listener(enclose!( (context) move |event: MouseMoveEvent| {
        context.fill_rect(f64::from(event.client_x() - 5), f64::from(event.client_y() - 5)
                          , 10.0, 10.0);
    }));

        stdweb::event_loop();
    }

    fn event_sender(&mut self, _event_sender: Sender<EventManager>) {}

    fn render_receiver(&mut self, _render_receiver: Receiver<Vec<RenderContainer>>) {}
}
