/// `Offset` is used to move an widget along the x- and y-axis.
#[derive(Default, Clone, Copy)]
pub struct Offset(pub f64, pub f64);

property!(Offset, OffsetProperty, offset, shared_offset);

impl From<(f64, f64)> for Offset {
    fn from(t: (f64, f64)) -> Self {
         Offset(t.0, t.1)
    }
}

impl From<f64> for Offset {
   fn from(t: f64) -> Self {
         Offset(t, t)
    }
}

// todo tests