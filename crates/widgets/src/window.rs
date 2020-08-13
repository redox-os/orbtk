use std::{collections::VecDeque, rc::Rc};

use crate::{api::prelude::*, proc_macros::*, shell::prelude::WindowRequest, theme::prelude::*};

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
        window(ctx.widget()).bounds_mut().set_size(width, height);
        window(ctx.widget())
            .constraint_mut()
            .set_size(width, height);
    }

    fn active_changed(&self, active: bool, ctx: &mut Context) {
        window(ctx.widget()).set_active(active);

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
        let focused_widget = ctx.widget().get::<Global>("global").focused_widget;

        if (focused_widget.is_some() && focused_widget.unwrap() == entity)
            || !*ctx.get_widget(entity).get::<bool>("enabled")
        {
            return;
        }

        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            let mut old_focused_element = ctx.get_widget(old_focused_element);

            old_focused_element.set("focused", false);
            old_focused_element
                .get_mut::<Selector>("selector")
                .clear_state();
            old_focused_element.update(false);
        }

        ctx.window().get_mut::<Global>("global").focused_widget = Some(entity);

        if ctx.get_widget(entity).has::<bool>("focused") {
            let mut focused_element = ctx.get_widget(entity);

            focused_element.set("focused", true);
            focused_element
                .get_mut::<Selector>("selector")
                .set_state("focused");
            focused_element.update(false);
        }
    }

    fn remove_focus(&self, entity: Entity, ctx: &mut Context) {
        if let Some(old_focused_element) = ctx.window().get::<Global>("global").focused_widget {
            if old_focused_element != entity {
                return;
            }
            let mut old_focused_element = ctx.get_widget(old_focused_element);
            old_focused_element.set("focused", false);
            old_focused_element
                .get_mut::<Selector>("selector")
                .clear_state();
            old_focused_element.update(false);
        }

        ctx.widget().get_mut::<Global>("global").focused_widget = None;
    }

    fn set_background(&mut self, ctx: &mut Context) {
        let background: Brush = ctx.widget().clone("background");
        match background {
            Brush::SolidColor(color) => {
                ctx.render_context_2_d().set_background(color);
            }
            _ => {}
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
        if self.background != *window(ctx.widget()).background() {
            self.set_background(ctx);
        }

        if !self.title.eq(window(ctx.widget()).title()) {
            self.title = window(ctx.widget()).clone_title();
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
    Window<WindowState> {
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
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(GridLayout::new())
    }
}
