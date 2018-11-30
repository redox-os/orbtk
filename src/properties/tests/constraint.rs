use super::*;

#[test]
fn test_perform() {
    let constraint = Constraint {
        min_width: 10,
        min_height: 10,
        max_width: 50,
        max_height: 60,
        width: 0,
        height: 0,
    };

    assert_eq!(constraint.perform((10, 59)), (10, 59));
    assert_eq!(constraint.perform((5, 40)), (10, 40));
    assert_eq!(constraint.perform((10, 70)), (10, 60));
}