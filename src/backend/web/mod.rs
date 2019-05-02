use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use dces::prelude::{Entity, World};
use orbgl_web::prelude::*;
use orbgl_api::prelude::*;

use orbtk_web_window::prelude::*;

use crate::prelude::*;
use crate::backend::*;

// todo will be removed after orbgl supports text rendering
pub struct DummyRenderer;

impl Renderer for DummyRenderer {
    fn render(&mut self, background: Color) {}

    fn render_rectangle(&mut self, bounds: &Bounds, parent_bounds: &Bounds, global_position: &Point, border_radius: u32, background: Color, border_width: u32, border_color: Color, opacity: f32) {}

    fn render_text(&mut self, text: &str, bounds: &Bounds, parent_bounds: &Bounds, global_position: &Point, font_size: u32, color: Color, font: &Font) {}

    fn render_image(&mut self, image: &[Color], bounds: &Bounds, parent_bounds: &Bounds, global_position: &Point) {}
}

/// Implementation of the OrbClient based backend.
pub struct WebBackend {
    inner: WebWindow,
    theme: Theme,
    event_queue: RefCell<EventQueue>,
    messages: RefCell<BTreeMap<Entity, Vec<MessageBox>>>,
    canvas: Canvas,
}

impl WebBackend {
    pub fn new(theme: Theme, inner: WebWindow) -> WebBackend {
        let mut inner = inner;

        let surface = WebSurface::new(inner.width() as u32, inner.height() as u32, canvas.get_context().unwrap());
        let render_engine = WebRenderEngine::new(surface);
        let canvas = Canvas::new(render_engine.clone());

        WebBackend {
            inner,
            theme,
            event_queue: RefCell::new(EventQueue::default()),
            messages: RefCell::new(BTreeMap::new()),
            canvas,
        }
    }
}

impl Drop for WebBackend {
    fn drop(&mut self) {
        self.inner.sync();
    }
}

impl Backend for WebBackend {
    fn drain_events(&mut self) {}

    fn init_context(&mut self) -> InitContext<'_> {
        InitContext { theme: &self.theme }
    }

    fn render_context(&mut self) -> RenderContext<'_> {
        RenderContext {
            canvas: &mut self.canvas,
            renderer: &mut self.inner,
            theme: &self.theme,
            event_queue: &self.event_queue,
        }
    }

    fn layout_context(&mut self) -> LayoutContext<'_> {
        LayoutContext {
            window_size: (self.inner.width() as u32, self.inner.height() as u32),
            theme: &self.theme,
        }
    }

    fn event_context(&mut self) -> EventContext<'_> {
        EventContext {
            event_queue: &self.event_queue,
        }
    }

    fn state_context(&mut self) -> StateContext<'_> {
        StateContext {
            theme: &self.theme,
            event_queue: &self.event_queue,
            messages: &mut self.messages,
        }
    }
}

/// Implementation of the Web based backend runner.
pub struct WebBackendRunner {
    pub world: Option<World<Tree>>,
    pub backend: Rc<RefCell<WebBackend>>,
}

impl BackendRunner for WebBackendRunner {
    fn world(&mut self, world: World<Tree>) {
        self.world = Some(world);
    }

    fn run(&mut self, update: Rc<Cell<bool>>, running: Rc<Cell<bool>>) {
        window().request_animation_frame(|| {
            if !running.get() {
                break;
            }

            if let Some(world) = &mut self.world {
                world.run();
            }

            update.set(false);

            self.backend.borrow_mut().drain_events();
        });
    }
}

pub fn target_backend(
    title: &str,
    bounds: Bounds,
    resizable: bool,
    theme: Theme,
) -> (Box<WebBackendRunner>, Rc<RefCell<dyn Backend>>) {
    let mut flags = vec![];


    let backend = Rc::new(RefCell::new(WebBackend::new(
        theme,
        WebWindow::create().title(title).size(bounds.x(), bounds.y()).build(),
    )));

    let backend_runner = Box::new(WebBackendRunner {
        backend: backend.clone(),
        world: None,
    });

    (backend_runner, backend)
}

pub struct DummyFontMeasure;

impl FontMeasure for DummyFontMeasure {
    fn measure(&self, text: &str, font: &Font, font_size: u32) -> (u32, u32) {
        if font_size == 0 {
            return (0, 0);
        }
        let text = font.render(text, font_size as f32);
        (text.width(), text.height())
    }
}

lazy_static! {
    pub static ref FONT_MEASURE: Arc<DummyFontMeasure> = { Arc::new(DummyFontMeasure) };
}