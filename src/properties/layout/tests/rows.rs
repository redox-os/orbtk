// todo: write tests.
use super::*;

#[test]
fn test_with_height() {
    let height = RowHeight::Height(64.0);

    let builder = RowBuilder::new();
    let row = builder.with_height(height).build();

    assert_eq!(row.height, height);
}

#[test]
fn test_with_min_height() {
    let min_height = 64.0;

    let builder = RowBuilder::new();
    let row = builder.with_min_height(min_height).build();

    assert_eq!(row.min_height, min_height);
}

#[test]
fn test_with_max_height() {
    let max_height = 64.0;

    let builder = RowBuilder::new();
    let row = builder.with_max_height(max_height).build();

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
        .with_min_height(min_height)
        .with_max_height(max_height)
        .build();

    row.set_current_height(out_one_height);
    assert_eq!(row.current_height(), min_height);

    row.set_current_height(out_two_height);
    assert_eq!(row.current_height(), max_height);

    row.set_current_height(in_height);
    assert_eq!(row.current_height(), in_height);
}

#[test]
fn test_with() {
    let height = RowHeight::Height(64.0);

    let builder = RowsBuilder::new();
    let rows = builder.build();

    assert_eq!(rows.len(), 0);

    let builder = RowsBuilder::new();
    let rows = builder
        .with(Row::create().build())
        .with(Row::create().build())
        .build();

    assert_eq!(rows.len(), 2);
}
