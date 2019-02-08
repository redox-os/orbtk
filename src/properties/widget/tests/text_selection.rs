use super::*;

#[test]
fn test_into() {
    let offset: TextSelection = (14, 16).into();
    assert_eq!(offset.start_index, 14);
    assert_eq!(offset.length, 16);
}