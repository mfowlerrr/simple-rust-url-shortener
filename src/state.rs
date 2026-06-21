use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct AppState {
    pub urls: Arc<Mutex<HashMap<String, String>>>,
}
