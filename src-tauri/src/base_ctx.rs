use std::{collections::HashMap, sync::Mutex};

pub struct BaseCtx {
    store: Mutex<HashMap<u64, String>>,
}

impl BaseCtx {
    pub fn init() -> Self {
        Self {
            store: Mutex::new(HashMap::new())
        }
    }
}
