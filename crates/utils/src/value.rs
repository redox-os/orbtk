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

impl Into<String> for Value {
    fn into(self) -> String {
        self.get::<String>()
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        self.get::<f64>()
    }
}

impl Into<f32> for Value {
    fn into(self) -> f32 {
        self.get::<f32>()
    }
}
