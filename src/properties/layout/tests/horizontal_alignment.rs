use super::*;
use crate::enums::Alignment;

#[test]
fn test_align_x() {
    let available_width = 100.0;
    let width = 50.0;
    let margin = Margin::default();

    let horizontal_alignment = HorizontalAlignment(Alignment::Stretch);
    assert_eq!(
        horizontal_alignment.align_x(available_width, width, margin),
        0.0
    );

    let horizontal_alignment = HorizontalAlignment(Alignment::Center);
    assert_eq!(
        horizontal_alignment.align_x(available_width, width, margin),
        25.0
    );

    let horizontal_alignment = HorizontalAlignment(Alignment::Start);
    assert_eq!(
        horizontal_alignment.align_x(available_width, width, margin),
        0.0
    );

    let horizontal_alignment = HorizontalAlignment(Alignment::End);
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

    let horizontal_alignment = HorizontalAlignment(Alignment::Stretch);
    assert_eq!(
        horizontal_alignment.align_width(available_width, width, margin),
        available_width
    );

    let horizontal_alignment = HorizontalAlignment(Alignment::Center);
    assert_eq!(
        horizontal_alignment.align_width(available_width, width, margin),
        width
    );

    let horizontal_alignment = HorizontalAlignment(Alignment::Start);
    assert_eq!(
        horizontal_alignment.align_width(available_width, width, margin),
        width
    );

    let horizontal_alignment = HorizontalAlignment(Alignment::End);
    assert_eq!(
        horizontal_alignment.align_width(available_width, width, margin),
        width
    );
}

#[test]
fn test_into() {
    let horizontal_alignment: HorizontalAlignment = "Start".into();
    assert_eq!(horizontal_alignment.0, Alignment::Start);

    let horizontal_alignment: HorizontalAlignment = "start".into();
    assert_eq!(horizontal_alignment.0, Alignment::Start);

    let horizontal_alignment: HorizontalAlignment = "Center".into();
    assert_eq!(horizontal_alignment.0, Alignment::Center);

    let horizontal_alignment: HorizontalAlignment = "center".into();
    assert_eq!(horizontal_alignment.0, Alignment::Center);

    let horizontal_alignment: HorizontalAlignment = "End".into();
    assert_eq!(horizontal_alignment.0, Alignment::End);

    let horizontal_alignment: HorizontalAlignment = "end".into();
    assert_eq!(horizontal_alignment.0, Alignment::End);

    let horizontal_alignment: HorizontalAlignment = "Stretch".into();
    assert_eq!(horizontal_alignment.0, Alignment::Stretch);

    let horizontal_alignment: HorizontalAlignment = "stretch".into();
    assert_eq!(horizontal_alignment.0, Alignment::Stretch);

    let horizontal_alignment: HorizontalAlignment = "other".into();
    assert_eq!(horizontal_alignment.0, Alignment::Stretch);
}
