use crate::entities::settings::{Settings, SettingsRepositoryInterface};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub struct SettingsRepository {
    file_path: String,
}

impl SettingsRepository {
    const FILE_NAME: &'static str = "my_settings.toml";

    pub fn new(mut app_path: PathBuf) -> Self {
        app_path.push(Self::FILE_NAME);
        let file_path = app_path
            .to_str()
            .expect("Failed to open database file path.")
            .to_string();
        Self { file_path }
    }
}

impl SettingsRepositoryInterface<Box<dyn std::error::Error>> for SettingsRepository {
    fn load(&self) -> Result<Settings, Box<dyn std::error::Error>> {
        let setting_raw = fs::read_to_string(&self.file_path)?;
        let setting = toml::from_str::<Settings>(&setting_raw)?;

        Ok(setting)
    }

    fn save(&self, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
        let setting_raw = toml::to_string(settings).unwrap();

        let mut file = File::create(&self.file_path)?;
        write!(file, "{}", setting_raw)?;
        file.flush()?;

        Ok(())
    }
}
