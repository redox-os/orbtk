use super::*;

#[test]
fn test_height() {
    let height = RowHeight::Height(64.0);

    let builder = RowBuilder::new();
    let row = builder.height(height).build();

    assert_eq!(row.height, height);
}

#[test]
fn test_min_height() {
    let min_height = 64.0;

    let builder = RowBuilder::new();
    let row = builder.min_height(min_height).build();

    assert_eq!(row.min_height, min_height);
}

#[test]
fn test_max_height() {
    let max_height = 64.0;

    let builder = RowBuilder::new();
    let row = builder.max_height(max_height).build();

    assert_eq!(row.max_height, max_height);
}

#[test]
fn test_set_current_height() {
    let out_one_height = 10.0;
    let out_two_height = 66.0;
    let in_height = 33.0;
    let min_height = 14.0;
    let max_height = 64.0;

    let builder = RowBuilder::new();
    let mut row = builder
        .min_height(min_height)
        .max_height(max_height)
        .build();

    row.set_current_height(out_one_height);
    assert_eq!(row.current_height(), min_height);

    row.set_current_height(out_two_height);
    assert_eq!(row.current_height(), max_height);

    row.set_current_height(in_height);
    assert_eq!(row.current_height(), in_height);
}

#[test]
fn test_row() {
    let height = RowHeight::Height(64.0);

    let builder = RowsBuilder::new();
    let rows = builder.build();

    assert_eq!(rows.len(), 0);

    let builder = RowsBuilder::new();
    let rows = builder
        .row(Row::create().build())
        .row(Row::create().build())
        .build();

    assert_eq!(rows.len(), 2);
}

#[test]
fn test_row_height_into() {
    let row: Row = "Auto".into();
    assert_eq!(row.height(), RowHeight::Auto);

    let row: Row = "auto".into();
    assert_eq!(row.height(), RowHeight::Auto);

    let row: Row = "Stretch".into();
    assert_eq!(row.height(), RowHeight::Stretch);

    let row: Row = "stretch".into();
    assert_eq!(row.height(), RowHeight::Stretch);

    let row: Row = "*".into();
    assert_eq!(row.height(), RowHeight::Stretch);

    let row: Row = "other".into();
    assert_eq!(row.height(), RowHeight::Stretch);

    let row: Row = 64.0.into();
    assert_eq!(row.height(), RowHeight::Height(64.0));
}
