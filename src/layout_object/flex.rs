use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use {Alignment, Constraint, Entity, EntityComponentManager, LayoutObject, LayoutResult, Theme};

pub struct FlexLayoutObject {
    orientation: Alignment,
    current_child: Cell<usize>,
    current_position: RefCell<Vec<u32>>,
}

impl FlexLayoutObject {
    pub fn new(orientation: Alignment) -> Self {
        FlexLayoutObject {
            orientation,
            current_child: Cell::new(0),
            current_position: RefCell::new(vec![]),
        }
    }
}

impl LayoutObject for FlexLayoutObject {
    fn layout(
        &self,
        _entity: Entity,
        _ecm: &EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        children_pos: &mut Option<HashMap<Entity, (i32, i32)>>,
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(size) = size {
            self.current_child.set(self.current_child.get() + 1);

            match self.orientation {
                Alignment::Horizontal => {
                    self.current_position.borrow_mut().push(size.0);
                }
                Alignment::Vertical => {
                    self.current_position.borrow_mut().push(size.1);
                }
            }

            if self.current_child.get() == children.len() {
                let mut counter = 0;

                for child in children {
                    if counter == self.current_position.borrow().len() {
                        break;
                    }

                    if let None = children_pos {
                        *children_pos = Some(HashMap::new());
                    }
                    if let Some(children_pos) = children_pos {
                        match self.orientation {
                            Alignment::Horizontal => {
                                children_pos.insert(
                                    *child,
                                    (self.current_position.borrow()[counter] as i32, 0),
                                );
                            }
                            Alignment::Vertical => {
                                children_pos.insert(
                                    *child,
                                    (0, self.current_position.borrow()[counter] as i32),
                                );
                            }
                        }

                        counter += 1;
                    }
                }

                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.min_width, constraint.min_height));
            }
            self.current_position.borrow_mut().clear();
            self.current_position.borrow_mut().push(0);
            self.current_child.set(0);
        }

        let child_bc = Constraint {
            min_width: constraint.min_width,
            max_width: constraint.max_width,
            min_height: constraint.min_height,
            max_height: constraint.max_height,
        };

        LayoutResult::RequestChild(children[self.current_child.get()], child_bc)
    }
}
