use std::rc::Rc;

use state::State;
use structs::Rect;
use widget::{Property, PropertyResult, Template, Widget, WidgetContainer};
use layout_object::{LayoutObject, StretchLayoutObject};

#[derive(Default, Clone, Copy)]
pub struct Offset(pub i32, pub i32);

#[derive(Default)]
pub struct ScrollViewerState;

impl State for ScrollViewerState {
    fn update(&self, widget: &mut WidgetContainer) {
        let bounds = {
            if let Ok(bounds) = widget.borrow_property::<Rect>() {
                bounds.clone()
            } else {
                Rect::default()
            }
        };

        let child_bounds = {
            if let Ok(bounds) = widget.borrow_child_property::<Rect>(0) {
                bounds.clone()
            } else {
                Rect::default()
            }
        };

        if let Ok(offset) = widget.borrow_mut_property::<Offset>() {
            offset.0 = ((bounds.width as i32 - child_bounds.width as i32)).min(0);
            offset.1 = ((bounds.height as i32 - child_bounds.height as i32)).min(0);
        }
    }
}

pub struct ScrollViewer {
    pub child: Option<Rc<Widget>>,
    pub offset: Property<Offset>,
    pub state: Rc<ScrollViewerState>,
}

impl Default for ScrollViewer {
    fn default() -> ScrollViewer {
        ScrollViewer {
            child: None,
            offset: Property::new(Offset::default()),
            state: Rc::new(ScrollViewerState),
        }
    }
}

impl Widget for ScrollViewer {
    fn template(&self) -> Template {
        print!("ScrollViewer -> ");
        if let Some(child) = &self.child {
            Template::Single(child.clone())
        } else {
            Template::Empty
        }
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.offset.build()]
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(StretchLayoutObject)
    }
}
