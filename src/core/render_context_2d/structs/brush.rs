

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Gradient {
}

// impl Default for Gradient {
//     fn default() -> Self {
//         Gradient {
//             color_stops: BTreeMap::new()
//         }
//     }
// }

// impl Gradient {
//     pub fn new() -> Self {
//         Gradient::default()
//     }

//     pub fn add_color_stop(&mut self)
// }


pub enum Brush {
    SolidColor(String),
    // Gradient([i32])
}