/// Compares two f64 values.
pub fn f64_assert(v1: f64, v2: f64) {
    let error_margin = f64::EPSILON;

    assert!((v1 - v2).abs() < error_margin)
}
