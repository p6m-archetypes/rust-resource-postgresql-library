use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersistenceSettings {
    pub url: String,
    pub max_connections: Option<u32>,
    pub run_migrations: Option<bool>,
}

impl Default for PersistenceSettings {
    fn default() -> Self {
        Self {
            url: url_from_env(),
            max_connections: None,
            run_migrations: Some(true),
        }
    }
}

// When deployed via the platform operator, individual connection fields are
// injected as DB_HOST / DB_PORT / DB_USERNAME / DB_PASSWORD / DB_DBNAME.
// Assemble a URL from those if present; fall back to a local dev default
// that can also be overridden via APP_PERSISTENCE__URL.
fn url_from_env() -> String {
    if let (Ok(host), Ok(port), Ok(user), Ok(pass), Ok(db)) = (
        std::env::var("DB_HOST"),
        std::env::var("DB_PORT"),
        std::env::var("DB_USERNAME"),
        std::env::var("DB_PASSWORD"),
        std::env::var("DB_DBNAME"),
    ) {
        return format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db);
    }
    "postgres://dev:dev@localhost/{{ prefix_name }}_{{ suffix_name }}".to_string()
}
