use serde::de::DeserializeOwned;
/// Wraps a ron value and is used to support conversion to different types.
pub struct Value(pub ron::Value);

impl Value {
    /// Converts the internal value to the given type.
    pub fn get<T>(self) -> T
    where
        T: Default + DeserializeOwned,
    {
        if let Ok(value) = self.0.into_rust::<T>() {
            return value;
        }

        T::default()
    }
}

impl From<ron::Value> for Value {
    fn from(v: ron::Value) -> Self {
        Value(v)
    }
}

impl From<Value> for String {
    fn from(v: Value) -> String {
        v.get::<String>()
    }
}

impl From<Value> for f64 {
    fn from(v: Value) -> f64 {
        v.get::<f64>()
    }
}

impl From<Value> for f32 {
    fn from(v: Value) -> f32 {
        v.get::<f32>()
    }
}
