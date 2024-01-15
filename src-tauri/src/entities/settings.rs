use serde::{Deserialize, Serialize};

pub trait SettingsRepositoryInterface<E> {
    fn load(&self) -> Result<Settings, E> ;
    fn save(&self, settings: &Settings) -> Result<(), E>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct General {
    pub store_path: Option<String>,
    pub language: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Appearance {
    pub theme: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub setting_version: Option<u8>,
    pub general: Option<General>,
    pub appearance: Option<Appearance>,
}

impl Settings {
    pub fn new_settings() -> Self {
        Self {
            setting_version: Some(1),
            general: Some(General {
                store_path: Some("../password.json".to_string()),
                language: Some("en-US".to_string()),
            }),
            appearance: Some(Appearance {
                theme: Some("light".to_string()),
            }),
        }
    }

    // changes で Someになっている値だけを更新
    pub fn merge(&self, changes: Self) -> Self {
        let mut base = self.clone();

        if let Some(setting_version) = changes.setting_version {
            base.setting_version = Some(setting_version);
        }

        if let Some(general) = changes.general {
            if let Some(store_path) = general.store_path {
                base.general
                    .as_mut()
                    .map(|g| g.store_path = Some(store_path));
            }
            if let Some(language) = general.language {
                base.general.as_mut().map(|g| g.language = Some(language));
            }
        }

        if let Some(appearance) = changes.appearance {
            if let Some(theme) = appearance.theme {
                base.appearance.as_mut().map(|a| a.theme = Some(theme));
            }
        }
        base
    }
}
