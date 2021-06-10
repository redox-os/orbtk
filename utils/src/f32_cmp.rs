/// Compares two f32 values.
pub fn f32_cmp(v1: f32, v2: f32) {
    let error_margin = f32::EPSILON;

    assert!((v1 - v2).abs() < error_margin)
}
