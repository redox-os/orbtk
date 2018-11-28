use std::cell::{Cell, RefCell};

use dces::{Entity, EntityComponentManager};
use layout_object::{LayoutObject, LayoutResult};
use theme::Theme;

use enums::Alignment;
use structs::{Constraint, Rect};

pub struct FlexLayoutObject {
    orientation: Alignment,
    current_child: Cell<usize>,
    current_position: RefCell<Vec<u32>>,
    width: Cell<u32>,
    height: Cell<u32>,
}

impl Into<Box<LayoutObject>> for FlexLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl FlexLayoutObject {
    pub fn new(orientation: Alignment) -> Self {
        FlexLayoutObject {
            orientation,
            current_child: Cell::new(0),
            current_position: RefCell::new(vec![]),
            width: Cell::new(0),
            height: Cell::new(0),
        }
    }
}

impl LayoutObject for FlexLayoutObject {
    fn layout(
        &self,
        _entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(size) = size {
            self.current_child.set(self.current_child.get() + 1);

            if self.current_child.get() <= children.len() {
                match self.orientation {
                    Alignment::Horizontal => {
                        self.current_position.borrow_mut().push(size.0);

                        if size.1 > self.height.get()
                            && size.1 <= constraint.max_height
                            && size.1 >= constraint.min_height
                        {
                            self.height.set(size.1);
                        }
                    }
                    Alignment::Vertical => {
                        self.current_position.borrow_mut().push(size.1);

                        if size.0 > self.width.get()
                            && size.0 <= constraint.max_width
                            && size.0 >= constraint.min_width
                        {
                            self.width.set(size.0);
                        }
                    }
                }
            }

            if self.current_child.get() == children.len() {
                let mut counter = 0;

                for child in children {
                    let mut current_pos = 0;

                    for i in 0..counter {
                        current_pos += self.current_position.borrow()[i];
                    }

                    if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(*child) {
                        match self.orientation {
                            Alignment::Horizontal => {
                                bounds.x = current_pos as i32;
                                bounds.y = 0;
                            }
                            Alignment::Vertical => {
                                bounds.x = 0;
                                bounds.y = current_pos as i32;
                            }
                        }

                        counter += 1;
                    }
                }

                match self.orientation {
                    Alignment::Horizontal => {
                        return LayoutResult::Size(constraint.perform((self.current_position.borrow().iter().sum(), self.height.get())));
                    }
                    Alignment::Vertical => {
                        return LayoutResult::Size(constraint.perform((self.width.get(), self.current_position.borrow().iter().sum())));
                    }
                }
            }
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.min_width, constraint.min_height));
            }
            self.current_position.borrow_mut().clear();
            self.current_child.set(0);
            self.width.set(0);
            self.height.set(0);
        }

        let child_bc = Constraint {
            min_width: constraint.min_width,
            max_width: constraint.max_width,
            min_height: constraint.min_height,
            max_height: constraint.max_height,
            width: constraint.width,
            height: constraint.height,
        };

        LayoutResult::RequestChild(children[self.current_child.get()], child_bc)
    }
}
