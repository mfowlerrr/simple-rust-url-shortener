use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub urls: Arc<Mutex<HashMap<String, String>>>,
}
