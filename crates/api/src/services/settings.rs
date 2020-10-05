#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs::{create_dir_all, File},
    io::Write,
};

#[cfg(not(target_arch = "wasm32"))]
use threadpool::*;

#[cfg(target_arch = "wasm32")]
use stdweb::web::window;

#[cfg(target_arch = "wasm32")]
use ron::de::from_str;

#[cfg(not(target_arch = "wasm32"))]
use ron::de::from_reader;

use ron::ser::{to_string_pretty, PrettyConfig};

use serde::{de::DeserializeOwned, Serialize};

use dces::entity::Entity;

use crate::widget_base::MessageAdapter;

pub enum SettingsMessage<D>
where
    D: DeserializeOwned,
{
    Saved,
    Loaded(D),
}

/// `Settings` represents a global settings service that could be use to serialize and deserialize
/// data in the `ron` file format. Settings are stored in the user settings directory (depending on the operating system)
/// under the a folder with the given application name.
#[derive(Debug, Clone)]
pub struct Settings {
    app_name: Box<str>,
    message_adapter: MessageAdapter,

    #[cfg(not(target_arch = "wasm32"))]
    pool: ThreadPool,
}

impl Settings {
    /// Creates a new `Settings` service with an default name.
    pub fn new(message_adapter: MessageAdapter) -> Self {
        Settings {
            app_name: "orbtk_app".into(),
            message_adapter,

            #[cfg(not(target_arch = "wasm32"))]
            pool: ThreadPool::new(4),
        }
    }
    /// Creates a new `Settings` service with the given app name.
    pub fn from_name(app_name: impl Into<Box<str>>, message_adapter: MessageAdapter) -> Self {
        Settings {
            app_name: app_name.into(),
            message_adapter,

            #[cfg(not(target_arch = "wasm32"))]
            pool: ThreadPool::new(4),
        }
    }

    /// Gets the app name of the setting service.
    pub fn app_name(&self) -> &str {
        &*self.app_name
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Serialize the given data object from user's config dir. Sends a saved message to the given entity.
    pub fn save_async<S: Serialize>(&self, key: &str, data: &S, entity: Entity) {
        self.pool.execute(move || {})
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Serialize the given data object from user's config dir.
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

    #[cfg(not(target_arch = "wasm32"))]
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

    #[cfg(target_arch = "wasm32")]
    /// Serialize the given data object from the browser storage.
    pub fn save<S: Serialize>(&self, key: &str, data: &S) -> Result<(), String> {
        let content = to_string_pretty(data, PrettyConfig::default());
        if window()
            .local_storage()
            .insert(key, content.unwrap().as_str())
            .is_ok()
        {
            return Ok(());
        }

        Err(format!(
            "Settings.save: Could not write settings with key {} to local browser storage.",
            key
        ))
    }

    #[cfg(target_arch = "wasm32")]
    /// Loads and deserialize data from the browser storage.
    pub fn load<D: DeserializeOwned>(&self, key: &str) -> Result<D, String> {
        if let Some(data) = window().local_storage().get(key) {
            if let Ok(data) = from_str(data.as_str()) {
                return Ok(data);
            } else {
                return Err(format!(
                    "Settings.load: Could not read data from local browser storage with key: {}",
                    key
                ));
            }
        }

        Err(format!(
            "Settings.load: Could not read data from local browser storage with key: {}",
            key
        ))
    }
}
