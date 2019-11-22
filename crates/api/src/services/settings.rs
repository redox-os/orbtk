use std::{
    fs::{create_dir_all, File},
    io::Write,
};

use dirs;
use serde::{de::DeserializeOwned, Serialize};

use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};

/// `Settings` represents a global settings service that could be use to serialize and deserialize
/// data in the `ron` file format. Settings are stored in the user settings directory (depending on the operation system)
/// under the a folder with the given application name.
pub struct Settings {
    app_name: Box<str>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            app_name: "orbtk-app".into(),
        }
    }
}

impl Settings {
    /// Creates a new `Settings` service with the given app name.
    pub fn new(app_name: impl Into<Box<str>>) -> Self {
        Settings {
            app_name: app_name.into(),
        }
    }

    /// Gets the app name of the setting service.
    pub fn app_name(&self) -> &str {
        &*self.app_name
    }

    /// Serialize the given data object and user's config dir.
    pub fn save<S: Serialize>(&self, key: &str, data: &S) -> Result<(), String> {
        let content = to_string_pretty(data, PrettyConfig::default());

        if let Some(config_path) = &mut dirs::config_dir() {
            config_path.push(&*self.app_name);

            if !config_path.exists() {
                let result = create_dir_all(&config_path);

                if result.is_err() {
                    return Err(format!(
                        "Settings.save: Could not create settings dir {:?}",
                        config_path
                    ));
                }
            }

            config_path.push(format!("{}.ron", key));

            if let Ok(file) = &mut File::create(&config_path) {
                let result = file.write_all(content.unwrap().as_bytes());
                if result.is_err() {
                    return Err(format!(
                        "Settings.save: Could not write to config file {:?}",
                        config_path
                    ));
                }
            } else {
                return Err(format!(
                    "Settings.save: Could not create config file {:?}",
                    config_path
                ));
            }
        }

        Ok(())
    }

    /// Loads and deserialize data from user's config dir.
    pub fn load<D: DeserializeOwned>(&self, key: &str) -> Result<D, String> {
        if let Some(config_path) = &mut dirs::config_dir() {
            config_path.push(&*self.app_name);
            config_path.push(format!("{}.ron", key));

            if let Ok(file) = &mut File::open(&config_path) {
               if let Ok(data) = from_reader(file) {
                   return Ok(data);
               } else {
                return Err(format!(
                    "Settings.load: Could not read data from config file {:?}",
                    config_path
                ));
               }
            } else {
                return Err(format!(
                    "Settings.load: Could not open config file {:?}",
                    config_path
                ));
            }
        }

        Err(format!(
            "Settings.load: Could not load settings with key: {}",
            key
        ))
    }
}
