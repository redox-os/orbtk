use super::*;

#[test]
fn test_align_position() {
    let available_measure = 100.0;
    let measure = 50.0;

    let alignment = Alignment::Stretch;
    assert_eq!(
        alignment.align_position(available_measure, measure, 0.0, 0.0),
        0.0
    );

    let alignment = Alignment::Center;
    assert_eq!(
        alignment.align_position(available_measure, measure, 0.0, 0.0),
        25.0
    );

    let alignment = Alignment::Start;
    assert_eq!(
        alignment.align_position(available_measure, measure, 0.0, 0.0),
        0.0
    );

    let alignment = Alignment::End;
    assert_eq!(
        alignment.align_position(available_measure, measure, 0.0, 0.0),
        50.0
    );
}

#[test]
fn test_align_measure() {
    let available_measure = 100.0;
    let measure = 50.0;

    let alignment = Alignment::Stretch;
    assert_eq!(
        alignment.align_measure(available_measure, measure, 0.0, 0.0),
        available_measure
    );

    let alignment = Alignment::Center;
    assert_eq!(
        alignment.align_measure(available_measure, measure, 0.0, 0.0),
        measure
    );

    let alignment = Alignment::Start;
    assert_eq!(
        alignment.align_measure(available_measure, measure, 0.0, 0.0),
        measure
    );

    let alignment = Alignment::End;
    assert_eq!(
        alignment.align_measure(available_measure, measure, 0.0, 0.0),
        measure
    );
}

#[test]
fn test_into() {
    let alignment: Alignment = "Start".into();
    assert_eq!(alignment, Alignment::Start);

    let alignment: Alignment = "start".into();
    assert_eq!(alignment, Alignment::Start);

    let alignment: Alignment = "Center".into();
    assert_eq!(alignment, Alignment::Center);

    let alignment: Alignment = "center".into();
    assert_eq!(alignment, Alignment::Center);

    let alignment: Alignment = "End".into();
    assert_eq!(alignment, Alignment::End);

    let alignment: Alignment = "end".into();
    assert_eq!(alignment, Alignment::End);

    let alignment: Alignment = "Stretch".into();
    assert_eq!(alignment, Alignment::Stretch);

    let alignment: Alignment = "stretch".into();
    assert_eq!(alignment, Alignment::Stretch);

    let alignment: Alignment = "other".into();
    assert_eq!(alignment, Alignment::Stretch);
}
