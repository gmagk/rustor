use std::collections::HashMap;
use std::{env, fs};
use std::io::Error;
use std::iter::Map;
use clap::Parser;
use serde::Deserialize;

#[derive(Default, Parser)]
#[command(name = "Rustor")]
pub struct Params {
    #[arg(long, required = false, help = "Location of configuration file.")]
    config_file: Option<String>,
}

#[derive(Default, Clone, Deserialize)]
pub struct ConfigValues {
    key_bindings: HashMap<ConfigKeyBindingKey, char>
}

#[derive(Clone)]
pub struct Config {
    values: ConfigValues
}

impl ConfigValues {

    pub fn key_bindings(&self) -> &HashMap<ConfigKeyBindingKey, char> {
        &self.key_bindings
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Deserialize)]
pub enum ConfigKeyBindingKey {
    KbAdd,
    KbDel,
    KbDownload,
    KbInfo,
    KbHelp,
    KbHome,
    KbOpen,
    KbQuit,
    KbReAnn,
    KbSearch
}

impl Config {

    pub fn new(params: Params) -> Self {

        // Read_config
        let config_file = match params.config_file {
            Some(file) => file,
            None => {
                let home_dir = env::home_dir()
                    .expect("Error: Tried to search for `$HOME/.rustor/config.toml` but user's Home directory was not found!")
                    .to_str()
                    .unwrap()
                    .to_string();
                let file = format!("{}/.rustor/config.toml", home_dir);
                match fs::exists(file.clone()) {
                    Ok(true) => file,
                    Ok(false) => "".to_string(),
                    Err(_) => "".to_string(),
                }
            }
        };
        let mut values: ConfigValues = if !config_file.is_empty() {
                match fs::read_to_string(config_file.clone()) {
                    Ok(content) => {
                        match toml::from_str(&content) {
                            Ok(vals) => Ok(vals),
                            Err(e) => Err(Error::other(format!("Failed to parse config file: {}", e)))
                        }
                    },
                    Err(_) => { Err(Error::other(format!("Failed to read config file: {}", config_file.clone()))) }
                }
            } else {
                Ok(ConfigValues::default())
            }.unwrap();

        // Add key bindings missing from config file
        let mut default_key_bindings = HashMap::new();
        default_key_bindings.insert(ConfigKeyBindingKey::KbAdd, 'a');
        default_key_bindings.insert(ConfigKeyBindingKey::KbDel, 'd');
        default_key_bindings.insert(ConfigKeyBindingKey::KbDownload, 'd');
        default_key_bindings.insert(ConfigKeyBindingKey::KbInfo, 'i');
        default_key_bindings.insert(ConfigKeyBindingKey::KbHelp, 'h');
        default_key_bindings.insert(ConfigKeyBindingKey::KbHome, 'b');
        default_key_bindings.insert(ConfigKeyBindingKey::KbQuit, 'q');
        default_key_bindings.insert(ConfigKeyBindingKey::KbOpen, 'o');
        default_key_bindings.insert(ConfigKeyBindingKey::KbReAnn, 'r');
        default_key_bindings.insert(ConfigKeyBindingKey::KbSearch, 's');
        let mut missing_key_bindings: HashMap<ConfigKeyBindingKey, char> = HashMap::new();
        default_key_bindings.iter().for_each(|(k, v)| {
            match values.key_bindings.iter().find(| (key, value) | **key == *k) {
                 Some(_) => {},
                 None => { let _ = missing_key_bindings.insert(k.clone(), v.clone()); }
             }
         });
        values.key_bindings.extend(missing_key_bindings.into_iter().map(|(k, v)| (k.clone(), v.clone())));

        Self { values }
    }

    pub fn values(&self) -> ConfigValues {
        self.values.clone()
    }
}
