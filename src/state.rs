use crate::models::ShortenReply;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct InMemoryStore {
    pub map: Arc<Mutex<HashMap<String, String>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub trait UrlStoreTrait {
    async fn insert(&self, code: String, url: String);
    async fn get(&self, code: &str) -> Option<String>;
    async fn get_all(&self) -> Vec<ShortenReply>;
}

impl UrlStoreTrait for InMemoryStore {
    async fn insert(&self, code: String, url: String) {
        self.map.lock().await.insert(code, url);
    }

    async fn get(&self, code: &str) -> Option<String> {
        self.map.lock().await.get(code).cloned()
    }

    async fn get_all(&self) -> Vec<ShortenReply> {
        self.map
            .lock()
            .await
            .iter()
            .map(|(k, v)| ShortenReply {
                code: k.clone(),
                url: v.clone(),
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub store: InMemoryStore,
}
