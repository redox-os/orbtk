use std::cell::RefCell;
use std::rc::Rc;

use dces::World;

use super::renderer::CanvasRenderer;
use backend::{Backend, BackendRunner, EventContext, LayoutContext, RenderContext};
use event::EventQueue;
use structs::Rect;
use theme::Theme;
use tree::Tree;

pub struct WasmBackend {
    running: bool,
    inner: CanvasRenderer,
    theme: Theme,
    event_queue: RefCell<EventQueue>,
}

impl WasmBackend {
    pub fn new(theme: Theme, inner: CanvasRenderer) -> WasmBackend {
        WasmBackend {
            inner,
            theme,
            event_queue: RefCell::new(EventQueue::default()),
            running: false,
        }
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
