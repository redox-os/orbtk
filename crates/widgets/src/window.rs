use std::{collections::VecDeque, rc::Rc};

use crate::{
    api::prelude::*, proc_macros::*, shell::prelude::WindowRequest, theme_default::prelude::*,
};

// --- KEYS --
pub static STYLE_WINDOW: &str = "window";
// --- KEYS --

// internal type to handle dirty widgets.
type DirtyWidgets = Vec<Entity>;

#[derive(Clone)]
enum Action {
    WindowEvent(WindowEvent),
    FocusEvent(FocusEvent),
}

// The `WindowState` handles the window events.
#[derive(Default, AsAny)]
struct WindowState {
    actions: VecDeque<Action>,
    background: Brush,
    title: String,
}

impl WindowState {
    fn push_action(&mut self, action: Action) {
        self.actions.push_front(action);
    }

    fn resize(&self, width: f64, height: f64, ctx: &mut Context) {
        Window::bounds_mut(&mut ctx.window()).set_size(width, height);
        Window::constraint_mut(&mut ctx.window()).set_size(width, height);
    }

    fn active_changed(&self, active: bool, ctx: &mut Context) {
        Window::active_set(&mut ctx.widget(), active);

        // if !active {
        //     // remove focus if the window is not active
        //     if let Some(focused_widget) = ctx.window().get::<Global>("global").focused_widget {
        //         ctx.window().get_mut::<Global>("global").focused_widget = None;
        //         if ctx.get_widget(focused_widget).has::<bool>("focused") {
        //             ctx.get_widget(focused_widget).set("focused", false);
        //             ctx.get_widget(focused_widget).update_theme_by_state(false);
        //         }
        //     }
        // }
    }

    fn request_focus(&self, entity: Entity, ctx: &mut Context) {
        let mut focus_state: FocusState = Window::focus_state_clone(&ctx.widget());
        focus_state.request_focus(entity, ctx);
        Window::focus_state_set(&mut ctx.widget(), focus_state);
    }

    fn remove_focus(&self, entity: Entity, ctx: &mut Context) {
        let mut focus_state: FocusState = Window::focus_state_clone(&ctx.widget());
        focus_state.remove_focus(entity, ctx);
        Window::focus_state_set(&mut ctx.widget(), focus_state);
    }

    fn set_background(&mut self, ctx: &mut Context) {
        let background: Brush = ctx.widget().clone("background");
        if let Brush::SolidColor(color) = background {
            ctx.render_context_2_d().set_background(color);
        };
        self.background = background;
    }
}

impl State for WindowState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.set_background(ctx);
        self.title = ctx.widget().clone("title");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.background != *Window::background_ref(&ctx.widget()) {
            self.set_background(ctx);
        }

        let window = ctx.widget();

        if !self.title.eq(Window::title_ref(&window)) {
            self.title = Window::title_clone(&window);
            ctx.send_window_request(WindowRequest::ChangeTitle(self.title.clone()));
        }

        if let Some(action) = self.actions.pop_front() {
            match action {
                Action::WindowEvent(window_event) => match window_event {
                    WindowEvent::Resize { width, height } => {
                        self.resize(width, height, ctx);
                    }
                    WindowEvent::ActiveChanged(active) => {
                        self.active_changed(active, ctx);
                    }
                    _ => {}
                },
                Action::FocusEvent(focus_event) => match focus_event {
                    FocusEvent::RequestFocus(entity) => {
                        self.request_focus(entity, ctx);
                    }
                    FocusEvent::RemoveFocus(entity) => {
                        self.remove_focus(entity, ctx);
                    }
                },
            }
        }
    }
}

widget!(
    /// The `Window` widget provides access to the properties of an application window.
    /// It also contains global properties like keyboard modifier and focused widget.
    ///
    /// **style:** `window`
    Window<WindowState>: ActivateHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the title property.
        title: String,

        /// Sets or shares the resizeable property.
        resizeable: bool,

        /// Sets or shares the property if this window should always be on top.
        always_on_top: bool,

        /// Sets or shares the flag if the window is borderless.
        borderless: bool,

        /// Sets or shares a value that describes if the current window is active.
        active: bool,

        /// Access the current keyboard state e.g. to check modifiers.
        keyboard_state: KeyboardState,

        /// Access the current window theme.
        theme: Theme,

        /// Access the current focus state.
        focus_state: FocusState,

        /// Internal property to handle dirty widgets.
        dirty_widgets: DirtyWidgets
    }
);

impl Window {
    fn on_window_event<H: Fn(&mut StatesContext, WindowEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(WindowEventHandler {
            handler: Rc::new(handler),
        })
    }

    fn on_focus_event<H: Fn(&mut StatesContext, FocusEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(FocusEventHandler {
            handler: Rc::new(handler),
        })
    }
}

impl Template for Window {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("Window")
            .background(colors::BRIGHT_GRAY_COLOR)
            .size(100.0, 100.0)
            .style(STYLE_WINDOW)
            .title("Window")
            .resizeable(false)
            .always_on_top(false)
            .on_window_event(move |ctx, event| {
                ctx.get_mut::<WindowState>(id)
                    .push_action(Action::WindowEvent(event));
                true
            })
            .on_focus_event(move |ctx, event| {
                ctx.get_mut::<WindowState>(id)
                    .push_action(Action::FocusEvent(event));
                true
            })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        GridLayout::new().into()
    }
}
