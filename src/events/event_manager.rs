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
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            focus_manager: FocusManager::new(),
            events: VecDeque::new(),
            mouse_over_widget: RefCell::new(None),
        }
    }

    pub fn push_back(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn drain_events(&mut self, widgets: &Vec<Arc<Widget>>) -> bool {
        let mut redraw = false;

        while let Some(event) = self.events.pop_front() {
            for widget in widgets {
                redraw = redraw || self.bubble_event(&event, widget);
                // if widget.event(event, self.focus_manager.focused(&widget), &mut self.redraw) {
                //     if !self.focus_manager.focused(&widget) {
                //         self.focus_manager.request_focus(&widget);
                //         self.redraw = true;
                //     }
                // }
            }
        }

        redraw
    }

    pub fn bubble_event(&self, event: &Event, widget: &Arc<Widget>) -> bool {
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
                widget.on_mouse_down(&args);
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
