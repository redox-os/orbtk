// use std::collections::HashMap;

// use {BoxConstraints, ComponentBox, Entity, EntityComponentManager, Layout, LayoutResult, Widget};

// pub struct Row;
// pub struct Column;

// #[derive(Default)]
// pub struct FlexProps {
//     direction: Axis,

//     // layout continuation state
//     ix: usize,
//     major_per_flex: u32,
//     minor: u32,
// }

// pub struct Flex;

// pub enum Axis {
//     Horizontal,
//     Vertical,
// }

// impl Default for Axis {
//     fn default() -> Axis {
//         Axis::Horizontal
//     }
// }

// impl Axis {
//     fn major(&self, coords: (u32, u32)) -> u32 {
//         match *self {
//             Axis::Horizontal => coords.0,
//             Axis::Vertical => coords.1,
//         }
//     }

//     fn minor(&self, coords: (u32, u32)) -> u32 {
//         match *self {
//             Axis::Horizontal => coords.1,
//             Axis::Vertical => coords.0,
//         }
//     }

//     fn pack(&self, major: u32, minor: u32) -> (u32, u32) {
//         match *self {
//             Axis::Horizontal => (major, minor),
//             Axis::Vertical => (minor, major),
//         }
//     }
// }

// // impl Row {
// //     pub fn new() -> Flex {
// //         Flex {
// //             direction: Axis::Horizontal,

// //             ix: 0,
// //             major_per_flex: 0,
// //             minor: 0,
// //         }
// //     }
// // }

// // impl Column {
// //     pub fn new() -> Flex {
// //         Flex {
// //             direction: Axis::Vertical,

// //             ix: 0,
// //             major_per_flex: 0,
// //             minor: 0,
// //         }
// //     }
// // }

// impl Widget for Flex {
//     fn components(&self) -> Vec<ComponentBox> {
//         vec![
//             ComponentBox::new(Axis::Horizontal),
//             // ComponentBox::new(Layout::new(Box::new(
//             //     |entity: Entity,
//             //      ecm: &EntityComponentManager,
//             //      bc: &BoxConstraints,
//             //      children: &[Entity],
//             //      children_pos: &mut HashMap<Entity, (i32, i32)>,
//             //      size: Option<(u32, u32)>| {
                    
//             //         if let Ok(props) = ecm.borrow_component::<FlexProps>(entity) {
//             //             let mut ix = props.ix;
//             //             let mut minor = props.minor;
//             //             let mut major_per_flex = props.major_per_flex;

//             //             if let Some(size) = size {
//             //                 let minor = props.direction.minor(size);
//             //                 minor = props.minor.max(minor);

                            
//             //                 ix += 1;
//             //                 if ix == children.len() {
//             //                     // measured all children
//             //                     let mut major = 0;
//             //                     for &child in children {
//             //                         // top-align, could do center etc. based on child height
//             //                         let pos = props.direction.pack(major, 0);
//             //                         children_pos.insert(child, (pos.0 as i32, pos.1 as i32));
//             //                         major += props.major_per_flex;
//             //                     }
//             //                     let max_major = props.direction.major((bc.max_width, bc.max_height));
//             //                     return LayoutResult::Size(
//             //                         props.direction.pack(max_major, minor),
//             //                     );
//             //                 }
//             //             } else {
//             //                 if children.is_empty() {
//             //                     return LayoutResult::Size((bc.min_width, bc.min_height));
//             //                 }
//             //                 ix = 0;
//             //                 minor = props.direction.minor((bc.min_width, bc.min_height));
//             //                 let max_major = props.direction.major((bc.max_width, bc.max_height));
//             //                 major_per_flex = max_major / (children.len() as u32);
//             //             }
//             //             let child_bc = match props.direction {
//             //                 Axis::Horizontal => BoxConstraints {
//             //                     min_width: major_per_flex,
//             //                     max_width: major_per_flex,
//             //                     min_height: bc.min_height,
//             //                     max_height: bc.max_height,
//             //                 },
//             //                 Axis::Vertical => BoxConstraints {
//             //                     min_width: bc.min_width,
//             //                     max_width: bc.max_width,
//             //                     min_height: props.major_per_flex,
//             //                     max_height: props.major_per_flex,
//             //                 },
//             //             };
//             //             return LayoutResult::RequestChild(children[ix], child_bc)
//             //         }

//             //         LayoutResult::Size((bc.min_width, bc.min_height))               
//             //     },
//             // ))),
//         ]
//     }
// }
