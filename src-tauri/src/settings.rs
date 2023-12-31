use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    pub store_path: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Appearance {
    pub theme: Option<Theme>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub setting_version: Option<u8>,
    pub general: Option<General>,
    pub appearance: Option<Appearance>,
}

// For public
impl Settings {
    const FILE_PATH: &'static str = "../my_settings.toml";

    pub fn new() -> Self {
        match fs::read_to_string(Self::FILE_PATH) {
            Ok(setting_raw) => {
                debug!("setting_raw: {:?}", &setting_raw);
                match toml::from_str::<Self>(&setting_raw) {
                    Ok(setting) => setting,
                    Err(e) => {
                        error!("There is an issue with the format of the configuration file.");
                        error!("{e}");
                        Self::new_settings()
                    }
                }
            }
            Err(e) => {
                error!("{e}");
                Self::new_settings()
            }
        }
    }

    pub fn submit(&self) -> Result<(), Box<dyn std::error::Error>> {
        let toml = toml::to_string(&self).unwrap();

        let mut file = File::create(Self::FILE_PATH)?;

        write!(file, "{}", toml)?;
        file.flush()?;

        Ok(())
    }

    // changes で Someになっている値だけを更新
    pub fn update(&mut self, changes: Self) {
        if let Some(setting_version) = changes.setting_version {
            self.setting_version = Some(setting_version);
        }

        if let Some(general) = changes.general {
            if let Some(store_path) = general.store_path {
                self.general
                    .as_mut()
                    .map(|g| g.store_path = Some(store_path));
            }
            if let Some(language) = general.language {
                self.general.as_mut().map(|g| g.language = Some(language));
            }
        }

        if let Some(appearance) = changes.appearance {
            if let Some(theme) = appearance.theme {
                self.appearance.as_mut().map(|a| a.theme = Some(theme));
            }
        }
    }
}

// For Private
impl Settings {
    fn new_settings() -> Self {
        Self {
            setting_version: Some(1),
            general: Some(General {
                store_path: Some("../password.json".to_string()),
                language: Some("en-US".to_string()),
            }),
            appearance: Some(Appearance {
                theme: Some(Theme::Light),
            }),
        }
    }
}
