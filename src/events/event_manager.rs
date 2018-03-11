use std::collections::VecDeque;
use std::sync::Arc;
use std::cell::RefCell;

use Event;
use FocusManager;
use Widget;
use Handleable;

pub struct EventManager {
    focus_manager: FocusManager,
    events: VecDeque<Event>,
    mouse_over_widget: RefCell<Option<Arc<Widget>>>,
    widget_stack: RefCell<Vec<Arc<Widget>>>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            focus_manager: FocusManager::new(),
            events: VecDeque::new(),
            mouse_over_widget: RefCell::new(None),
            widget_stack: RefCell::new(Vec::new()),
        }
    }

    pub fn push_back(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn drain_events(&mut self, widgets: &Vec<Arc<Widget>>) -> bool {
        let mut redraw = false;
        self.widget_stack.borrow_mut().clear();

        while let Some(event) = self.events.pop_front() {
            for widget in widgets {
                match event.clone() {
                    Event::KeyDownEvent(args) => {
                        self.build_key_event_stack(&event, widget);
                        // if event stack is not empty tunnle and bubble evnet
                    }
                    Event::KeyUpEvent(args) => {}
                    _ => {}
                }

                redraw = redraw || self.tunnel_event(&event, widget);
            }

            while let Some(widget) = self.widget_stack.borrow_mut().pop() {
                redraw = redraw || self.bubble_event(&event, &widget);
            }
        }

        redraw
    }

    fn build_key_event_stack(&self, event: &Event, widget: &Arc<Widget>) {
        if self.focus_manager.focused(widget) {
            self.widget_stack.borrow_mut().push(widget.clone());
            return;
        }

        // if the widget is a leaf and it's not focused the widget stack will cleared
        if widget.children().borrow().len() == 0 {
            self.widget_stack.borrow_mut().clear();
        }

        for child in widget.children().borrow().iter() {
            self.build_key_event_stack(event, child);
        }
    }

    fn tunnel_event(&self, event: &Event, widget: &Arc<Widget>) -> bool {
        let mut redraw = false;
        let mut push_to_event_stack = false;

        match event.clone() {
            Event::MouseDownEvent(args) => {
                if !widget.rect().get().contains(args.point) {
                    return false;
                }
                widget.on_preview_mouse_down(&args, &self.focus_manager);
                if args.handled().get() {
                    return true;
                } else {
                    push_to_event_stack = true;
                }
            }
            Event::ScrollEvent(args) => {
                if !widget.rect().get().contains(args.point) {
                    return false;
                }

                widget.on_preview_scroll(&args);
                if args.handled().get() {
                    return true;
                } else {
                    push_to_event_stack = true;
                }
            }
            _ => {}
        }

        if push_to_event_stack {
            self.widget_stack.borrow_mut().push(widget.clone());
            for child in widget.children().borrow().iter() {
                redraw = self.tunnel_event(event, child)
            }
        }

        redraw
    }

    fn bubble_event(&self, event: &Event, widget: &Arc<Widget>) -> bool {
        match event.clone() {
            Event::KeyDownEvent(args) => {
                if !self.focus_manager.focused(widget) {
                    return false;
                }
                widget.on_key_down(&args);
                return args.handled().get();
            }
            Event::KeyUpEvent(args) => {
                widget.on_key_up(&args);
                return args.handled().get();
            }
            Event::MouseDownEvent(args) => {
                if !widget.rect().get().contains(args.point) {
                    return false;
                }
                widget.on_mouse_down(&args, &self.focus_manager);
                return args.handled().get();
            }
            Event::MouseUpEvent(args) => {
                widget.on_mouse_up(&args);
                return args.handled().get();
            }
            Event::MouseMoveEvent(args) => {
                let mut mouse_leave = false;

                if let Some(ref mouse_over_widget) = *self.mouse_over_widget.borrow() {
                    if Arc::ptr_eq(mouse_over_widget, widget) {
                        widget.on_mouse_leave(&args);
                        mouse_leave = false;
                    }
                }

                if mouse_leave {
                    *self.mouse_over_widget.borrow_mut() = None;
                }

                if widget.rect().get().contains(args.point) {
                    widget.on_mouse_enter(&args);
                    *self.mouse_over_widget.borrow_mut() = Some(widget.clone());
                }

                return args.handled().get();
            }
            Event::ScrollEvent(args) => {
                if !widget.rect().get().contains(args.point) {
                    return false;
                }

                widget.on_scroll(&args);
                return args.handled().get();
            }
            _ => {}
        }

        false
    }
}
