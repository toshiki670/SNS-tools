use crate::entities::settings::{Settings, SettingsRepositoryInterface};
use log::{debug, error};

pub fn run<E: std::fmt::Debug>(repository: &dyn SettingsRepositoryInterface<E>, mut current: Settings, changes: Settings) -> bool {
    let new = current.merge(changes);
    debug!("New settings: {:?}", &new);
    if let Err(e) = repository.save(&new) {
        error!("Failed to save: {:?}", &e);
        false
    } else {
        true
    }
}
