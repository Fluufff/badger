use std::{env, sync::Arc};

use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use sqlx::MySqlPool;
use thiserror::Error;
use tower_http::services::ServeDir;
use tracing::{error, info};

mod auth;
mod endpoints;
mod templates;
mod types;

#[derive(Debug, Error)]
enum AppError {
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("OTP error: {0}")]
    Thotp(#[from] thotp::ThotpError),
    #[error("Template error: {0}")]
    Askama(#[from] askama::Error),
    #[error("Unauthorized")]
    Unauthorized,
    // #[error("other: {0}")]
    // Other(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("app error {}", self);
        match self {
            Self::Db(_) | Self::Io(_) | Self::Image(_) | Self::Askama(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::Jwt(_) | Self::Bcrypt(_) | Self::Thotp(_) => {
                StatusCode::UNAUTHORIZED.into_response()
            }
            Self::Unauthorized => Redirect::temporary("/login").into_response(),
            // Self::Other(reason) => (StatusCode::BAD_REQUEST, reason).into_response(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();
    let database_host = env::var("MARIADB_HOSTNAME").expect("MARIADB_HOSTNAME env required");
    let database_pass =
        env::var("MARIADB_ROOT_PASSWORD").expect("MARIADB_ROOT_PASSWORD env required");
    let database_name = env::var("MARIADB_DATABASE").expect("MARIADB_DATABASE env required");

    let database_url = format!("mysql://root:{database_pass}@{database_host}/{database_name}");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET env required");

    let db = MySqlPool::connect(&database_url).await?;

    let state = types::AppState { db, jwt_secret };

    let uploads_dir = std::env::var("UPLOADS_DIR").unwrap();
    let app = Router::new()
        .route(
            "/",
            get(endpoints::main_handler).post(endpoints::main_handler),
        )
        .route(
            "/designer",
            get(endpoints::designer::get_handler).post(endpoints::designer::post_handler),
        )
        .route(
            "/login",
            get(endpoints::auth::login_handler).post(endpoints::auth::login_handler),
        )
        .nest_service(&format!("/{}", &uploads_dir), ServeDir::new(&uploads_dir))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("[::]:3000").await.unwrap();
    info!("listening on [::]:3000");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
