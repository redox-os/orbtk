// use crate::structs::{Spacer, Thickness};

// use super::*;

// #[test]
// fn test_set_left() {
//     let left = 10.0;

//     let mut border_thickness = BorderThickness::default();
//     border_thickness.set_left(left);

//     assert_eq!(border_thickness.left(), left);
// }

// #[test]
// fn test_set_top() {
//     let top = 10.0;

//     let mut border_thickness = BorderThickness::default();
//     border_thickness.set_top(top);

//     assert_eq!(border_thickness.top(), top);
// }

// #[test]
// fn test_set_right() {
//     let right = 10.0;

//     let mut border_thickness = BorderThickness::default();
//     border_thickness.set_right(right);

//     assert_eq!(border_thickness.right(), right);
// }

// #[test]
// fn test_set_bottom() {
//     let bottom = 10.0;

//     let mut border_thickness = BorderThickness::default();
//     border_thickness.set_bottom(bottom);

//     assert_eq!(border_thickness.bottom(), bottom);
// }

// #[test]
// fn test_set_thickness() {
//     let thickness = Thickness {
//         left: 10.0,
//         right: 12.0,
//         top: 14.0,
//         bottom: 4.0,
//     };

//     let mut border_thickness = BorderThickness::default();
//     border_thickness.set_thickness(thickness);

//     assert_eq!(border_thickness.thickness(), thickness);
// }

// #[test]
// fn test_into() {
//     let border_thickness: BorderThickness = (10.0, 12.0, 13.0, 14.0).into();

//     assert_eq!(border_thickness.left(), 10.0);
//     assert_eq!(border_thickness.top(), 12.0);
//     assert_eq!(border_thickness.right(), 13.0);
//     assert_eq!(border_thickness.bottom(), 14.0);

//     let border_thickness: BorderThickness = 10.0.into();

//     assert_eq!(border_thickness.left(), 10.0);
//     assert_eq!(border_thickness.top(), 10.0);
//     assert_eq!(border_thickness.right(), 10.0);
//     assert_eq!(border_thickness.bottom(), 10.0);
// }
