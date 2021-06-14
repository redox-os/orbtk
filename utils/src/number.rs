use std::ops::Neg;

/// Valid number types (64bit)
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Number {
    Real(i64),
    Float(f64),
}

impl Default for Number {
    fn default() -> Self {
        Self::Real(0)
    }
}

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Float(n) => Number::Float(-n),
            Number::Real(n) => Number::Real(-n),
        }
    }
}

macro_rules! impl_float {
    ($t:ty) => {
        impl From<$t> for Number {
            fn from(n: $t) -> Number {
                Number::Float(n as f64)
            }
        }

        impl From<Number> for $t {
            fn from(n: Number) -> Self {
                match n {
                    Number::Real(n) => n as $t,
                    Number::Float(n) => n as $t,
                }
            }
        }
    };
}

macro_rules! impl_real {
    ($t:ty) => {
        impl From<$t> for Number {
            fn from(n: $t) -> Number {
                Number::Real(n as i64)
            }
        }

        impl From<Number> for $t {
            fn from(n: Number) -> $t {
                match n {
                    Number::Real(n) => n as $t,
                    Number::Float(n) => n as $t,
                }
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

impl_real!(u8);
impl_real!(i8);
impl_real!(u16);
impl_real!(i16);
impl_real!(u32);
impl_real!(i32);
impl_real!(u64);
impl_real!(i64);
