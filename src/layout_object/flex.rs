use std::collections::HashMap;
use std::sync::Arc;
use std::cell::Cell;

use {
    Alignment, Constraint, Entity, EntityComponentManager, LayoutObject, LayoutResult, Theme
};

pub struct FlexLayoutObject {
    pub direction: Alignment,

    // layout continuation state
    pub ix: Cell<usize>,
    pub major_per_flex: Cell<u32>,
    pub minor: Cell<u32>,
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
        _theme: &Arc<Theme>,
    ) -> LayoutResult {
        if let Some(size) = size {
            let minor = self.direction.minor(size);
            self.minor.set(self.minor.get().max(minor));
            self.ix.set(self.ix.get() + 1);
            if self.ix.get() == children.len() {
                // measured all children
                let mut major = 0;
                for child in children {
                    // top-align, could do center etc. based on child height

                    if let None = children_pos {
                        *children_pos = Some(HashMap::new());
                    }
                    if let Some(children_pos) = children_pos {
                        let pos = self.direction.pack(major, 0);
                        children_pos.insert(*child, (pos.0 as i32, pos.1 as i32));
                    }

                    major += self.major_per_flex.get();
                }
                let max_major = self.direction.major((constraint.max_width as u32, constraint.max_height as u32));
                return LayoutResult::Size(self.direction.pack(max_major, self.minor.get()));
            }
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.min_width, constraint.min_height));
            }
            self.ix.set(0);
            self.minor.set(self.direction.minor((constraint.min_width, constraint.min_height)));
            let max_major = self.direction.major((constraint.max_width, constraint.max_height));
            self.major_per_flex.set(max_major / children.len() as u32);
        }
        let child_bc = match self.direction {
            Alignment::Horizontal => Constraint {
                min_width: self.major_per_flex.get(),
                max_width: self.major_per_flex.get(),
                min_height: constraint.min_height,
                max_height: constraint.max_height,
            },
            Alignment::Vertical => Constraint {
                min_width: constraint.min_width,
                max_width: constraint.max_width,
                min_height: self.major_per_flex.get(),
                max_height: self.major_per_flex.get(),
            },
        };
        LayoutResult::RequestChild(children[self.ix.get()], child_bc)
    }
}
