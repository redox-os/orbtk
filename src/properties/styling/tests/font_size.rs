use crate::prelude::*;

#[test]
fn test_into() {
    let border_radius: BorderRadius = 20.0.into();
    assert_eq!(border_radius.0, 20.0);
}
