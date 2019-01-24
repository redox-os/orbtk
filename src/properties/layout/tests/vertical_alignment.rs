use super::*;

#[test]
fn test_align_y() {
    let available_height = 100.0;
    let height = 50.0;
    let margin = Margin::default();

    let vertical_alignment = VerticalAlignment::Stretch;
    assert_eq!(
        vertical_alignment.align_y(available_height, height, margin),
        0.0
    );

    let vertical_alignment = VerticalAlignment::Center;
    assert_eq!(
        vertical_alignment.align_y(available_height, height, margin),
        25.0
    );

    let vertical_alignment = VerticalAlignment::Top;
    assert_eq!(
        vertical_alignment.align_y(available_height, height, margin),
        0.0
    );

    let vertical_alignment = VerticalAlignment::Bottom;
    assert_eq!(
        vertical_alignment.align_y(available_height, height, margin),
        50.0
    );
}

#[test]
fn test_align_height() {
    let available_height = 100.0;
    let height = 50.0;
    let margin = Margin::default();

    let vertical_alignment = VerticalAlignment::Stretch;
    assert_eq!(
        vertical_alignment.align_height(available_height, height, margin),
        available_height
    );

    let vertical_alignment = VerticalAlignment::Center;
    assert_eq!(
        vertical_alignment.align_height(available_height, height, margin),
        height
    );

    let vertical_alignment = VerticalAlignment::Top;
    assert_eq!(
        vertical_alignment.align_height(available_height, height, margin),
        height
    );

    let vertical_alignment = VerticalAlignment::Bottom;
    assert_eq!(
        vertical_alignment.align_height(available_height, height, margin),
        height
    );
}
