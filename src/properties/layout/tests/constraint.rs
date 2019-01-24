use super::*;

#[test]
fn test_set_width() {
    let width = 12.0;

    let mut constraint = Constraint::default();
    constraint.set_width(width);
    assert_eq!(constraint.width(), width);
}

#[test]
fn test_set_height() {
    let height = 12.0;

    let mut constraint = Constraint::default();
    constraint.set_height(height);

    assert_eq!(constraint.height(), height);
}

#[test]
fn test_set_size() {
    let width = 12.0;
    let height = 14.0;

    let mut constraint = Constraint::default();
    constraint.set_size(width, height);

    assert_eq!(constraint.size(), (width, height));
}

#[test]
fn test_set_min_width() {
    let min_width = 12.0;

    let mut constraint = Constraint::default();
    constraint.set_min_width(min_width);

    assert_eq!(constraint.min_width(), min_width);
}

#[test]
fn test_set_min_height() {
    let min_height = 12.0;

    let mut constraint = Constraint::default();
    constraint.set_min_height(min_height);

    assert_eq!(constraint.min_height(), min_height);
}

#[test]
fn test_set_min_size() {
    let min_width = 12.0;
    let min_height = 14.0;

    let mut constraint = Constraint::default();
    constraint.set_min_size(min_width, min_height);

    assert_eq!(constraint.min_size(), (min_width, min_height));
}

#[test]
fn test_set_max_width() {
    let max_width = 12.0;

    let mut constraint = Constraint::default();
    constraint.set_max_width(max_width);

    assert_eq!(constraint.max_width(), max_width);
}

#[test]
fn test_set_max_height() {
    let max_height = 12.0;

    let mut constraint = Constraint::default();
    constraint.set_max_height(max_height);

    assert_eq!(constraint.max_height(), max_height);
}

#[test]
fn test_set_max_size() {
    let max_width = 12.0;
    let max_height = 14.0;

    let mut constraint = Constraint::default();
    constraint.set_max_size(max_width, max_height);

    assert_eq!(constraint.max_size(), (max_width, max_height));
}

#[test]
fn test_perform() {
    let mut constraint = Constraint::default();

    constraint.set_min_width(10.0);
    constraint.set_min_height(10.0);
    constraint.set_max_width(50.0);
    constraint.set_max_height(60.0);
    constraint.set_width(0.0);
    constraint.set_height(0.0);

    assert_eq!(constraint.perform((10.0, 59.0)), (10.0, 59.0));
    assert_eq!(constraint.perform((5.0, 40.0)), (10.0, 40.0));
    assert_eq!(constraint.perform((10.0, 70.0)), (10.0, 60.0));
}
