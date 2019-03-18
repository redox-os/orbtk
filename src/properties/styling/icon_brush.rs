// use crate::structs::{Brush, Color};

// /// Used to draw the icon brush of a widget.
// #[derive(Clone)]
// pub struct IconBrush(pub Brush);

// property!(IconBrush, IconBrushProperty, icon_brush, icon_brush_prop);

// impl From<IconBrush> for Color {
//     fn from(b: IconBrush) -> Color {
//         b.0.into()
//     }
// }

// impl Default for IconBrush {
//     fn default() -> IconBrush {
//         "#000000".into()
//     }
// }

// impl From<&str> for IconBrush {
//     fn from(s: &str) -> IconBrush {
//         IconBrush(s.into())
//     }
// }
