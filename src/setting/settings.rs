use anyhow::Result;
use config::{Config, Environment};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LogSettings {
    pub log_level: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct VocabSettings {
    pub words_path: String,
    pub dictionary_path: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Settings {
    pub vocab: VocabSettings,
    pub logging: LogSettings,
}

impl Settings {
    pub fn new(location: &str, env_prefix: &str) -> Result<Self> {
        let mut builder = Config::builder();

        if Path::new(location).exists() {
            builder = builder.add_source(config::File::with_name(location));
        } else {
            log::warn!("Configuration file not found");
        }

        builder = builder.add_source(
            Environment::with_prefix(env_prefix)
                .separator("__")
                .prefix_separator("__"),
        );

        let settings = builder.build()?.try_deserialize()?;

        Ok(settings)
    }

    pub fn json_pretty(&self) -> String {
        to_string_pretty(&self).expect("Failed serialize")
    }
}
