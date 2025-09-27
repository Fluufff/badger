use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    // pub assets_dir: PathBuf,
    pub jwt_secret: String,
}

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub badge_ids: Vec<i64>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub regnumber: i64,
    // pub email: String,
    pub hash: String,
    pub nick: Option<String>,
    pub gid: i32,
    // pub double_auth: Option<i32>,
    pub double_auth_secret: Option<String>,
    // pub locked: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Layer {
    id: i64,
    name: String,
    file_path: String,
    condition: serde_json::Value,
}
