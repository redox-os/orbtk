/// `Settings` represents a global settings service that could be use to serialize and deserialize
/// data in the `ron` file format. Settings are stored in the user settings directory under the
/// a folder with the given application name.
pub struct Settings {
    app_name: String
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            app_name: "orbtk-app".into()
        }
    }
}

impl Settings {
    /// Creates a new `Settings` service with the given app name.
    pub fn new(app_name: impl Into<String>) -> Self {
        Settings {
            app_name: app_name.into()
        }
    }

    /// Gets the app name of the setting service.
    pub fn app_name(&self) -> &String {
        &self.app_name
    }

    pub fn serialize(&self) {

    }

    pub fn deserialize(&self) {
        
    }
}