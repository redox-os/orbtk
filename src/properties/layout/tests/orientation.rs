use super::*;

#[test]
fn test_into() {
    let vertical_alignment : Orientation = "Vertical".into();
    assert_eq!(vertical_alignment, Orientation::Vertical);

    let vertical_alignment : Orientation = "vertical".into();
    assert_eq!(vertical_alignment, Orientation::Vertical);

    let vertical_alignment : Orientation = "Horizontal".into();
    assert_eq!(vertical_alignment, Orientation::Horizontal);

    let vertical_alignment : Orientation = "horizontal".into();
    assert_eq!(vertical_alignment, Orientation::Horizontal);

    let vertical_alignment : Orientation = "other".into();
    assert_eq!(vertical_alignment, Orientation::Vertical);
}