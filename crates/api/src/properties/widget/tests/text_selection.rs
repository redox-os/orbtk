use super::*;

#[test]
fn test_into() {
    let offset: TextSelection = (14, 16).into();
    assert_eq!(offset.0.start_index, 14);
    assert_eq!(offset.0.length, 16);
}
