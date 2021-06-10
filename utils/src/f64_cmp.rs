/// Compares two f64 values.
pub fn f64_cmp(v1: f64, v2: f64) -> bool {
    let error_margin = f64::EPSILON;

    (v1 - v2).abs() < error_margin
}
