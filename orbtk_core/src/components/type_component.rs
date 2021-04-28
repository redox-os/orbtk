use std::any;

#[derive(Clone, Debug)]
pub struct TypeComponent {
    type_id: any::TypeId,
    type_name: String,
}

impl TypeComponent {
    pub fn new<T: 'static>() -> Self {
        TypeComponent {
            type_id: any::TypeId::of::<T>(),
            type_name: any::type_name::<T>().to_string(),
        }
    }

    pub fn type_id(&self) -> any::TypeId {
        self.type_id
    }

    pub fn type_name(&self) -> &str {
        self.type_name.as_str()
    }
}
