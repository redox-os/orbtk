use super::*;

#[test]
fn test_align_x() {
    let available_width = 100.0;
    let width = 50.0;
    let margin = Margin::default();

    let horizontal_alignment = HorizontalAlignment::Stretch;
    assert_eq!(
        horizontal_alignment.align_x(available_width, width, margin),
        0.0
    );

    let horizontal_alignment = HorizontalAlignment::Center;
    assert_eq!(
        horizontal_alignment.align_x(available_width, width, margin),
        25.0
    );

    let horizontal_alignment = HorizontalAlignment::Left;
    assert_eq!(
        horizontal_alignment.align_x(available_width, width, margin),
        0.0
    );

    let horizontal_alignment = HorizontalAlignment::Right;
    assert_eq!(
        horizontal_alignment.align_x(available_width, width, margin),
        50.0
    );
}

#[test]
fn test_align_width() {
    let available_width = 100.0;
    let width = 50.0;
    let margin = Margin::default();

    let horizontal_alignment = HorizontalAlignment::Stretch;
    assert_eq!(
        horizontal_alignment.align_width(available_width, width, margin),
        available_width
    );

    let horizontal_alignment = HorizontalAlignment::Center;
    assert_eq!(
        horizontal_alignment.align_width(available_width, width, margin),
        width
    );

    let horizontal_alignment = HorizontalAlignment::Left;
    assert_eq!(
        horizontal_alignment.align_width(available_width, width, margin),
        width
    );

    let horizontal_alignment = HorizontalAlignment::Right;
    assert_eq!(
        horizontal_alignment.align_width(available_width, width, margin),
        width
    );
}
