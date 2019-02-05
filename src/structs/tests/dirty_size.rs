use super::*;

#[test]
fn test_set_width() {
    let width = 10.0;

    let mut dirty_size = DirtySize::default();

    dirty_size.set_width(width);

    assert_eq!(dirty_size.width(), width);
    assert!(dirty_size.dirty());
}

#[test]
fn test_set_height() {
    let height = 10.0;

    let mut dirty_size = DirtySize::default();
    dirty_size.set_height(height);

    assert_eq!(dirty_size.height(), height);
    assert!(dirty_size.dirty());
}

#[test]
fn test_set_size() {
    let size = (10.0, 20.0);

    let mut dirty_size = DirtySize::default();

    dirty_size.set_size(size.0, size.1);

    assert_eq!(dirty_size.size(), size);
    assert!(dirty_size.dirty());
}

#[test]
fn test_set_dirty() {
    let mut dirty_size = DirtySize::default();

    dirty_size.set_dirty(false);

    assert!(!dirty_size.dirty());
}
