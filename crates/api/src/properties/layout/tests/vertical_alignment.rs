use crate::{prelude::*, utils::*};

#[test]
fn test_align_y() {
    let available_height = 100.0;
    let height = 50.0;
    let margin = Margin::default();

    let vertical_alignment = VerticalAlignment(Alignment::Stretch);
    assert_eq!(
        vertical_alignment.align_y(available_height, height, margin.clone()),
        0.0
    );

    let vertical_alignment = VerticalAlignment(Alignment::Center);
    assert_eq!(
        vertical_alignment.align_y(available_height, height, margin.clone()),
        25.0
    );

    let vertical_alignment = VerticalAlignment(Alignment::Start);
    assert_eq!(
        vertical_alignment.align_y(available_height, height, margin.clone()),
        0.0
    );

    let vertical_alignment = VerticalAlignment(Alignment::End);
    assert_eq!(
        vertical_alignment.align_y(available_height, height, margin.clone()),
        50.0
    );
}

#[test]
fn test_align_height() {
    let available_height = 100.0;
    let height = 50.0;
    let margin = Margin::default();

    let vertical_alignment = VerticalAlignment(Alignment::Stretch);
    assert_eq!(
        vertical_alignment.align_height(available_height, height, margin.clone()),
        available_height
    );

    let vertical_alignment = VerticalAlignment(Alignment::Center);
    assert_eq!(
        vertical_alignment.align_height(available_height, height, margin.clone()),
        height
    );

    let vertical_alignment = VerticalAlignment(Alignment::Start);
    assert_eq!(
        vertical_alignment.align_height(available_height, height, margin.clone()),
        height
    );

    let vertical_alignment = VerticalAlignment(Alignment::End);
    assert_eq!(
        vertical_alignment.align_height(available_height, height, margin.clone()),
        height
    );
}

#[test]
fn test_into() {
    let vertical_alignment: VerticalAlignment = "start".into();
    assert_eq!(vertical_alignment.0, Alignment::Start);

    let vertical_alignment: VerticalAlignment = "start".into();
    assert_eq!(vertical_alignment.0, Alignment::Start);

    let vertical_alignment: VerticalAlignment = "Center".into();
    assert_eq!(vertical_alignment.0, Alignment::Center);

    let vertical_alignment: VerticalAlignment = "center".into();
    assert_eq!(vertical_alignment.0, Alignment::Center);

    let vertical_alignment: VerticalAlignment = "end".into();
    assert_eq!(vertical_alignment.0, Alignment::End);

    let vertical_alignment: VerticalAlignment = "end".into();
    assert_eq!(vertical_alignment.0, Alignment::End);

    let vertical_alignment: VerticalAlignment = "Stretch".into();
    assert_eq!(vertical_alignment.0, Alignment::Stretch);

    let vertical_alignment: VerticalAlignment = "stretch".into();
    assert_eq!(vertical_alignment.0, Alignment::Stretch);

    let vertical_alignment: VerticalAlignment = "other".into();
    assert_eq!(vertical_alignment.0, Alignment::Stretch);
}
