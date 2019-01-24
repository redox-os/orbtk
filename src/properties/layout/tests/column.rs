// todo: write tests.
use super::*;

#[test]
fn test_with_width() {
    let width = ColumnWidth::Width(64.0);

    let builder = ColumnBuilder::new();
    let column = builder.with_width(width).build();

    assert_eq!(column.width, width);
}

#[test]
fn test_with_min_width() {
    let min_width = 64.0;

    let builder = ColumnBuilder::new();
    let column = builder.with_min_width(min_width).build();

    assert_eq!(column.min_width, min_width);
}

#[test]
fn test_with_max_width() {
    let max_width = 64.0;

    let builder = ColumnBuilder::new();
    let column = builder.with_max_width(max_width).build();

    assert_eq!(column.max_width, max_width);
}

#[test]
fn test_set_current_width() {
    let out_one_width = 10.0;
    let out_two_width = 66.0;
    let in_width = 33.0;
    let min_width = 14.0;
    let max_width = 64.0;

    let builder = ColumnBuilder::new();
    let mut column = builder
        .with_min_width(min_width)
        .with_max_width(max_width)
        .build();

    column.set_current_width(out_one_width);
    assert_eq!(column.current_width(), min_width);

    column.set_current_width(out_two_width);
    assert_eq!(column.current_width(), max_width);

    column.set_current_width(in_width);
    assert_eq!(column.current_width(), in_width);
}

#[test]
fn test_with() {
    let width = ColumnWidth::Width(64.0);

    let builder = ColumnsBuilder::new();
    let columns = builder.build();

    assert_eq!(columns.len(), 0);

    let builder = ColumnsBuilder::new();
    let columns = builder
        .with(Column::create().build())
        .with(Column::create().build())
        .build();

    assert_eq!(columns.len(), 2);
}
