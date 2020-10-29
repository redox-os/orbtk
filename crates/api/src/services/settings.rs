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

#[derive(Debug)]
pub enum SettingsError {
    Saved(String),
    Loaded(String),
}

pub type SettingsResult<T> = Result<T, SettingsError>;

/// `Settings` represents a global settings service that could be use to serialize and deserialize
/// data in the `ron` file format. Settings are stored in the user settings directory (depending on the operating system)
/// under the a folder with the given application name.
#[derive(Debug, Clone)]
pub struct Settings {
    app_name: String,
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
    pub fn from_name(app_name: String, message_adapter: MessageAdapter) -> Self {
        Settings {
            app_name,
            message_adapter,

            #[cfg(not(target_arch = "wasm32"))]
            pool: ThreadPool::new(4),
        }
    }

    /// Gets the app name of the setting service.
    pub fn app_name(&self) -> &str {
        &*self.app_name
    }

    /// Serialize the given data object from user's config dir. Sends the result `Result<(), String>` as message to the given entity.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_async<S: Serialize + Send + Sync + 'static>(
        &self,
        key: String,
        data: S,
        entity: Entity,
    ) {
        let app_name = self.app_name.clone();
        let message_adapter = self.message_adapter.clone();

        self.pool.execute(move || {
            message_adapter.send_message(save(app_name.as_str(), key.as_str(), &data), entity);
        })
    }

    /// Loads and deserialize data from user's config dir. Send the result `Result<D, String>` as message to the given entity.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_async<D: DeserializeOwned + Send + Sync + 'static>(
        &self,
        key: String,
        entity: Entity,
    ) {
        let app_name = self.app_name.clone();
        let message_adapter = self.message_adapter.clone();

        self.pool.execute(move || {
            message_adapter.send_message(load::<D>(app_name.as_str(), key.as_str()), entity);
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_async<S: Serialize + Send + Sync + 'static>(
        &self,
        key: String,
        data: S,
        entity: Entity,
    ) {
        self.message_adapter
            .send_message(save(self.app_name.as_str(), key.as_str(), &data), entity);
    }

    /// Loads and deserialize data from user's config dir. Send the result `Result<D, String>` as message to the given entity.
    #[cfg(target_arch = "wasm32")]
    pub fn load_async<D: DeserializeOwned + Send + Sync + 'static>(
        &self,
        key: String,
        entity: Entity,
    ) {
        self.message_adapter
            .send_message(load::<D>(self.app_name.as_str(), key.as_str()), entity);
    }

    /// Serialize the given data object from user's config dir.
    pub fn save<S: Serialize>(&self, key: &str, data: &S) -> SettingsResult<()> {
        save(self.app_name.as_str(), key, data)
    }

    /// Loads and deserialize data from user's config dir.
    pub fn load<D: DeserializeOwned>(&self, key: &str) -> SettingsResult<D> {
        load(self.app_name.as_str(), key)
    }
}

// --- Helper --

#[cfg(not(target_arch = "wasm32"))]
fn save<S: Serialize>(app_name: &str, key: &str, data: &S) -> SettingsResult<()> {
    let content = to_string_pretty(data, PrettyConfig::default());

    if let Some(config_path) = &mut dirs_next::config_dir() {
        config_path.push(app_name);

        if !config_path.exists() {
            let result = create_dir_all(&config_path);

            if result.is_err() {
                return Err(SettingsError::Saved(format!(
                    "Settings.save: Could not create settings dir {:?}",
                    config_path
                )));
            }
        }

        config_path.push(format!("{}.ron", key));

        if let Ok(file) = &mut File::create(&config_path) {
            let result = file.write_all(content.unwrap().as_bytes());
            if result.is_err() {
                return Err(SettingsError::Saved(format!(
                    "Settings.save: Could not write to config file {:?}",
                    config_path
                )));
            }
        } else {
            return Err(SettingsError::Saved(format!(
                "Settings.save: Could not create config file {:?}",
                config_path
            )));
        }
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn load<D: DeserializeOwned>(app_name: &str, key: &str) -> SettingsResult<D> {
    if let Some(config_path) = &mut dirs_next::config_dir() {
        config_path.push(app_name);
        config_path.push(format!("{}.ron", key));

        if let Ok(file) = &mut File::open(&config_path) {
            if let Ok(data) = from_reader(file) {
                return Ok(data);
            } else {
                return Err(SettingsError::Loaded(format!(
                    "Settings.load: Could not read data from config file {:?}",
                    config_path
                )));
            }
        } else {
            return Err(SettingsError::Loaded(format!(
                "Settings.load: Could not open config file {:?}",
                config_path
            )));
        }
    }

    Err(SettingsError::Loaded(format!(
        "Settings.load: Could not load settings with key: {}",
        key
    )))
}

#[cfg(target_arch = "wasm32")]
fn save<S: Serialize>(_app_name: &str, key: &str, data: &S) -> SettingsResult<()> {
    let content = to_string_pretty(data, PrettyConfig::default());
    if window()
        .local_storage()
        .insert(key, content.unwrap().as_str())
        .is_ok()
    {
        return Ok(());
    }

    Err(SettingsError::Saved(format!(
        "Settings.save: Could not write settings with key {} to local browser storage.",
        key
    )))
}

#[cfg(target_arch = "wasm32")]
fn load<D: DeserializeOwned>(_app_name: &str, key: &str) -> SettingsResult<D> {
    if let Some(data) = window().local_storage().get(key) {
        if let Ok(data) = from_str(data.as_str()) {
            return Ok(data);
        } else {
            return Err(SettingsError::Loaded(format!(
                "Settings.load: Could not read data from local browser storage with key: {}",
                key
            )));
        }
    }

    Err(SettingsError::Loaded(format!(
        "Settings.load: Could not read data from local browser storage with key: {}",
        key
    )))
}

// --- Helper --
