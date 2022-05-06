use std::collections::HashMap;
use std::sync::Arc;

use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use actix_web::cookie::time::Duration as CookieDuration;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use tokio::sync::RwLock;

use crate::session::utils::generate_session_key;

type SessionState = HashMap<String, String>;

#[derive(Clone)]
pub struct InMemorySessionStore {
    map: Arc<RwLock<HashMap<String, (DateTime<Local>, SessionState)>>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait(?Send)]
impl SessionStore for InMemorySessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let get = {
            let read = self.map.read().await;
            read.get(session_key.as_ref()).cloned()
        };
        if let Some((t, s)) = get {
            let now = Local::now();
            if t < now {
                {
                    let mut write = self.map.write().await;
                    write.remove(session_key.as_ref());
                }
                Ok(None)
            } else {
                Ok(Some(s.clone()))
            }
        } else {
            Ok(None)
        }
    }

    async fn save(
        &self,
        session_state: SessionState,
        ttl: &CookieDuration,
    ) -> Result<SessionKey, SaveError> {
        let session_key = generate_session_key();
        let key = session_key.as_ref().to_string();
        let t = get_expire_datetime(ttl);
        {
            let mut write = self.map.write().await;
            write.insert(key, (t, session_state));
        }
        Ok(session_key)
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &CookieDuration,
    ) -> Result<SessionKey, UpdateError> {
        let key = session_key.as_ref().to_string();
        let t = get_expire_datetime(ttl);
        {
            let mut write = self.map.write().await;
            write.insert(key, (t, session_state));
        }
        Ok(session_key)
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), anyhow::Error> {
        let mut write = self.map.write().await;
        write.remove(session_key.as_ref());
        Ok(())
    }
}

fn get_expire_datetime(ttl: &CookieDuration) -> DateTime<Local> {
    let secs = ttl.whole_seconds();
    Local::now() + chrono::Duration::seconds(secs)
}
