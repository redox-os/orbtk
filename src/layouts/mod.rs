use {Point, Id, Constraint};

pub use self::center::Center;
pub use self::container::Container;
pub use self::row::Row;

mod center;
mod container;
mod row;

pub enum LayoutResult {
    Size(Point),
    RequestChild(Id, Constraint),
}

// fn layout(&mut self, bc: &BoxConstraints, root: Id) {
//         fn layout_rec(widgets: &mut [Box<Widget>], ctx: &mut LayoutCtx, graph: &Graph,
//             bc: &BoxConstraints, node: Id) -> (f32, f32)
//         {
//             let mut size = None;
//             loop {
//                 let layout_res = widgets[node].layout(bc, &graph.children[node], size, ctx);
//                 match layout_res {
//                     LayoutResult::Size(size) => {
//                         ctx.geom[node].size = size;
//                         return size;
//                     }
//                     LayoutResult::RequestChild(child, child_bc) => {
//                         size = Some(layout_rec(widgets, ctx, graph, &child_bc, child));
//                     }
//                 }
//             }
//         }

//         layout_rec(&mut self.widgets, &mut self.c, &self.graph, bc, root);
// }

// impl LayoutCtx {
//     pub fn dwrite_factory(&self) -> &directwrite::Factory {
//         &self.dwrite_factory
//     }

//     pub fn position_child(&mut self, child: Id, pos: (f32, f32)) {
//         self.geom[child].pos = pos;
//     }

//     pub fn get_child_size(&self, child: Id) -> (f32, f32) {
//         self.geom[child].size
//     }
// }


// impl LayoutCtx {
//     pub fn dwrite_factory(&self) -> &directwrite::Factory {
//         &self.dwrite_factory
//     }

//     pub fn position_child(&mut self, child: Id, pos: (f32, f32)) {
//         self.geom[child].pos = pos;
//     }

//     pub fn get_child_size(&self, child: Id) -> (f32, f32) {
//         self.geom[child].size
//     }
// }

// Label

//  fn layout(&mut self, bc: &BoxConstraints, _children: &[Id], _size: Option<(f32, f32)>,
//         _ctx: &mut LayoutCtx) -> LayoutResult
//     {
//         // TODO: measure text properly
//         LayoutResult::Size(bc.constrain((100.0, 17.0)))
// }

// Button


//     fn layout(&mut self, bc: &BoxConstraints, children: &[Id], size: Option<(f32, f32)>,
//         ctx: &mut LayoutCtx) -> LayoutResult
//     {
//         self.label.layout(bc, children, size, ctx)
// }

// Flex

// pub struct Flex {
//     direction: Axis,

//     // layout continuation state
//     ix: usize,
//     major_per_flex: f32,
//     minor: f32,
// }

// pub enum Axis {
//     Horizontal,
//     Vertical,
// }

// impl Axis {
//     fn major(&self, coords: (f32, f32)) -> f32 {
//         match *self {
//             Axis::Horizontal => coords.0,
//             Axis::Vertical => coords.1,
//         }
//     }

//     fn minor(&self, coords: (f32, f32)) -> f32 {
//         match *self {
//             Axis::Horizontal => coords.1,
//             Axis::Vertical => coords.0,
//         }
//     }

//     fn pack(&self, major: f32, minor: f32) -> (f32, f32) {
//         match *self {
//             Axis::Horizontal => (major, minor),
//             Axis::Vertical => (minor, major),
//         }
//     }
// }

// impl Row {
//     pub fn new() -> Flex {
//         Flex {
//             direction: Axis::Horizontal,

//             ix: 0,
//             major_per_flex: 0.0,
//             minor: 0.0,
//         }
//     }
// }

// impl Column {
//     pub fn new() -> Flex {
//         Flex {
//             direction: Axis::Vertical,

//             ix: 0,
//             major_per_flex: 0.0,
//             minor: 0.0,
//         }
//     }
// }

// impl Flex {
//     pub fn ui(self, children: &[Id], ctx: &mut UiInner) -> Id {
//         ctx.add(self, children)
//     }
// }

// impl Widget for Flex {
//     fn layout(&mut self, bc: &BoxConstraints, children: &[Id], size: Option<(f32, f32)>,
//         ctx: &mut LayoutCtx) -> LayoutResult
//     {
//         if let Some(size) = size {
//             let minor = self.direction.minor(size);
//             self.minor = self.minor.max(minor);
//             self.ix += 1;
//             if self.ix == children.len() {
//                 // measured all children
//                 let mut major = 0.0;
//                 for &child in children {
//                     // top-align, could do center etc. based on child height
//                     ctx.position_child(child, self.direction.pack(major, 0.0));
//                     major += self.major_per_flex;
//                 }
//                 let max_major = self.direction.major((bc.max_width, bc.max_height));
//                 return LayoutResult::Size(self.direction.pack(max_major, self.minor));
//             }
//         } else {
//             if children.is_empty() {
//                 return LayoutResult::Size((bc.min_width, bc.min_height));
//             }
//             self.ix = 0;
//             self.minor = self.direction.minor((bc.min_width, bc.min_height));
//             let max_major = self.direction.major((bc.max_width, bc.max_height));
//             self.major_per_flex = max_major / (children.len() as f32);
//         }
//         let child_bc = match self.direction {
//             Axis::Horizontal => BoxConstraints {
//                 min_width: self.major_per_flex,
//                 max_width: self.major_per_flex,
//                 min_height: bc.min_height,
//                 max_height: bc.max_height,
//             },
//             Axis::Vertical => BoxConstraints {
//                 min_width: bc.min_width,
//                 max_width: bc.max_width,
//                 min_height: self.major_per_flex,
//                 max_height: self.major_per_flex,
//             },
//         };
//         LayoutResult::RequestChild(children[self.ix], child_bc)
//     }
// }

// /// The context given to layout methods.
// pub struct LayoutCtx {
//     dwrite_factory: directwrite::Factory,

//     handle: WindowHandle,

//     /// Bounding box of each widget. The position is relative to the parent.
//     geom: Vec<Geometry>,

//     /// Queue of events to distribute to listeners
//     event_q: Vec<(Id, Box<Any>)>,
// }