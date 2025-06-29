use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::plugins::Plugin;

pub struct AppState {
    pub plugins: Arc<Mutex<Vec<Plugin>>>,
    // TODO: Если не чистить мёртвые сессии, будет утекать память,
    // но поскольку это прототип, то не страшно)
    pub sessions: Arc<Mutex<HashMap<uuid::Uuid, serde_json::Value>>>,
}
