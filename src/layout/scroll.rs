// use std::{
//     cell::{Cell, RefCell},
//     collections::BTreeMap,
//     f64,
//     rc::Rc,
// };

// use dces::prelude::{Entity, EntityComponentManager};

// use crate::{
//     application::Tree,
//     properties::{
//         Bounds, Constraint, HorizontalAlignment, Margin, Offset, Padding, VerticalAlignment,
//         Visibility,
//     },
//     structs::{DirtySize, Position, Size},
//     theme::Theme,
// };

// use super::Layout;

// /// IMPORTANT: The scroll layout will only work for the text box now. A update will follow!!!!
// #[derive(Default)]
// pub struct ScrollLayout {
//     old_child_size: Cell<(f64, f64)>,
//     desired_size: RefCell<DirtySize>,
//     old_offset: Cell<(f64, f64)>,
//     old_alignment: Cell<(VerticalAlignment, HorizontalAlignment)>,
// }

// impl ScrollLayout {
//     pub fn new() -> Self {
//         ScrollLayout::default()
//     }
// }

// impl Layout for ScrollLayout {
//     fn measure(
//         &self,
//         entity: Entity,
//         ecm: &mut EntityComponentManager,
//         tree: &Tree,
//         layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
//         theme: &Theme,
//     ) -> DirtySize {
//         if Visibility::get(entity, ecm) == Visibility::Collapsed {
//             self.desired_size.borrow_mut().set_size(0.0, 0.0);
//             return self.desired_size.borrow().clone();
//         }

//         let horizontal_alignment = HorizontalAlignment::get(entity, ecm);
//         let vertical_alignment = VerticalAlignment::get(entity, ecm);

//         if horizontal_alignment != self.old_alignment.get().1
//             || vertical_alignment != self.old_alignment.get().0
//         {
//             self.desired_size.borrow_mut().set_dirty(true);
//         }

//         let constraint = Constraint::get(entity, ecm);

//         if constraint.width() > 0.0 {
//             self.desired_size.borrow_mut().set_width(constraint.width());
//         }

//         if constraint.height() > 0.0 {
//             self.desired_size
//                 .borrow_mut()
//                 .set_height(constraint.height());
//         }

//         for child in &tree.children[&entity] {
//             if let Some(child_layout) = layouts.borrow().get(child) {
//                 let dirty = child_layout
//                     .measure(*child, ecm, tree, layouts, theme)
//                     .dirty()
//                     || self.desired_size.borrow().dirty();

//                 self.desired_size.borrow_mut().set_dirty(dirty);
//             }
//         }

//         if let Ok(off) = ecm.borrow_component::<Offset>(entity) {
//             if self.old_offset.get().0 != off.0 || self.old_offset.get().1 != off.1 {
//                 self.old_offset.set((off.0, off.1));
//                 self.desired_size.borrow_mut().set_dirty(true);
//             }
//         }

//         self.desired_size.borrow().clone()
//     }

//     fn arrange(
//         &self,
//         parent_size: (f64, f64),
//         entity: Entity,
//         ecm: &mut EntityComponentManager,
//         tree: &Tree,
//         layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
//         theme: &Theme,
//     ) -> (f64, f64) {
//         if !self.desired_size.borrow().dirty() {
//             return self.desired_size.borrow().size();
//         }

//         let horizontal_alignment = HorizontalAlignment::get(entity, ecm);
//         let vertical_alignment = VerticalAlignment::get(entity, ecm);
//         let margin = Margin::get(entity, ecm);
//         let _padding = Padding::get(entity, ecm);
//         let constraint = Constraint::get(entity, ecm);

//         let size = constraint.perform((
//             horizontal_alignment.align_width(
//                 parent_size.0,
//                 self.desired_size.borrow().width(),
//                 margin,
//             ),
//             vertical_alignment.align_height(
//                 parent_size.1,
//                 self.desired_size.borrow().height(),
//                 margin,
//             ),
//         ));

//         if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
//             bounds.set_width(size.0);
//             bounds.set_height(size.1);
//         }

//         // let mut vertical_scroll_mode = ScrollMode::default();
//         // let mut horizontal_scroll_mode = ScrollMode::default();

//         // if let Ok(mode) = ecm.borrow_component::<ScrollViewerMode>(entity) {
//         //     vertical_scroll_mode = mode.vertical;
//         //     horizontal_scroll_mode = mode.horizontal;
//         // }

//         let mut offset = (0.0, 0.0);

//         let old_child_size = self.old_child_size.get();

//         if let Ok(off) = ecm.borrow_component::<Offset>(entity) {
//             // off.0 = (center_size.0 as i32 - size.0 as i32).min(0);
//             // off.1 = (center_size.1 as i32 - size.1 as i32).min(0);

//             offset = (off.0, off.1);
//         }

//         for child in &tree.children[&entity] {
//             // let child_margin = get_margin(*child, ecm);
//             let mut child_size = old_child_size;
//             let child_vertical_alignment = VerticalAlignment::get(*child, ecm);
//             let child_margin = Margin::get(*child, ecm);

//             if let Some(child_layout) = layouts.borrow().get(child) {
//                 child_size =
//                     child_layout.arrange((f64::MAX, f64::MAX), *child, ecm, tree, layouts, theme);
//             }

//             if child_size.0 > size.0 {
//                 offset.0 = (offset.0 + old_child_size.0 - child_size.0).min(0.0);
//             } else {
//                 offset.0 = 0.0;
//             }

//             if let Ok(child_bounds) = ecm.borrow_mut_component::<Bounds>(*child) {
//                 child_bounds.set_x(offset.0);
//                 child_bounds.set_y(child_vertical_alignment.align_y(
//                     size.1,
//                     child_bounds.height(),
//                     child_margin,
//                 ));
//             }

//             if let Ok(off) = ecm.borrow_mut_component::<Offset>(entity) {
//                 off.0 = offset.0;
//                 off.1 = offset.1;
//             }

//             self.old_child_size.set(child_size);
//         }

//         self.desired_size.borrow_mut().set_dirty(false);
//         size
//     }
// }

// impl Into<Box<dyn Layout>> for ScrollLayout {
//     fn into(self) -> Box<dyn Layout> {
//         Box::new(self)
//     }
// }
