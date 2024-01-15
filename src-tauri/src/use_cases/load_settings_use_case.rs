use crate::entities::settings::{Settings, SettingsRepositoryInterface};

pub fn run<E>(repository: &dyn SettingsRepositoryInterface<E>) -> Settings {
    if let Ok(settings) = repository.load() {
        settings
    } else {
        Settings::new_settings()
    }
}