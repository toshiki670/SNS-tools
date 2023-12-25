use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    pub store_path: String,
    pub language: Language,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Language {
    EnUS,
    JaJP,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Appearance {
    pub theme: Theme,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Setting {
    pub setting_version: u8,
    pub general: General,
    pub appearance: Appearance,
}

impl Setting {
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
                        Self::new_setting()
                    }
                }
            }
            Err(e) => {
                error!("{e}");
                Self::new_setting()
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
}

impl Setting {
    fn new_setting() -> Self {
        Self {
            setting_version: 1,
            general: General {
                store_path: "../password.json".to_string(),
                language: Language::EnUS,
            },
            appearance: Appearance {
                theme: Theme::Light,
            },
        }
    }
}
