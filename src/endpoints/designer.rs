use std::{collections::HashMap, sync::Arc};

use crate::{
    AppError,
    types::{self},
};
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use axum_extra::extract::{CookieJar, Multipart};
use std::fs;
use tracing::warn;

pub async fn post_handler(
    cookies: CookieJar,
    State(state): State<Arc<types::AppState>>,
    // Form(data): Form<HashMap<String, String>>,
    mut multipart: Multipart,
    // req: Request
) -> Result<Response, AppError> {
    let (cookies, _) = super::auth::must_be_logged_in(cookies, state.as_ref())?;
    let mut data = HashMap::new();
    let mut name_font_file = None;
    let mut nr_font_file = None;
    let uploads_dir = std::env::var("UPLOADS_DIR").unwrap();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();

        if name == "name_font" || name == "nr_font" || name == "file" {
            let bytes = field.bytes().await.unwrap().to_vec();
            if bytes.len() == 0 {
                continue;
            }
            let ext = match &bytes {
                bytes if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) => "jpg",
                bytes if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) => {
                    "png"
                }
                bytes if bytes.starts_with(&[0x00, 0x01, 0x00, 0x00, 0x00]) => "ttf",
                bytes if bytes.starts_with(&[0x4F, 0x54, 0x54, 0x4F]) => "otf",
                _ => return Ok((StatusCode::BAD_REQUEST, "invalid file").into_response()),
            };
            let hash = md5::compute(&bytes);
            let hash = base16ct::lower::encode_string(&hash.0);
            let file_path = format!("{}.{}", hash, ext);
            fs::write(format!("{}/{}", uploads_dir, file_path), bytes).unwrap();
            if name == "name_font" {
                name_font_file = Some(file_path);
            } else {
                nr_font_file = Some(file_path);
            }
        } else {
            let value = field.text().await.unwrap_or_default();
            data.insert(name, value);
        }
    }

    let max_row = sqlx::query_scalar::<_, Option<i32>>("select max(id) from layers;")
        .fetch_one(&state.db)
        .await
        .map_err(AppError::from)?
        .unwrap_or_default();
    match data.get("row").and_then(|r| str::parse::<i32>(r).ok()) {
        None => match (
            data.get("page_width_mm")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("page_height_mm")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("dpi").and_then(|s| str::parse::<f32>(s).ok()),
            data.get("avatar_size_pt")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("avatar_y_pt")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("regnum_size")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("regnum_x_pt")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("regnum_y_pt")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("regnum_color"),
            data.get("nick_size")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("nick_y_pt")
                .and_then(|s| str::parse::<f32>(s).ok()),
            data.get("nick_color"),
        ) {
            (
                Some(page_width_mm),
                Some(page_height_mm),
                Some(dpi),
                Some(avatar_size_pt),
                Some(avatar_y_pt),
                Some(regnum_size),
                Some(regnum_x_pt),
                Some(regnum_y_pt),
                Some(regnum_color),
                Some(nick_size),
                Some(nick_y_pt),
                Some(nick_color),
            ) => {
                sqlx::query("update layers_config set page_width_mm=?, page_height_mm=?, dpi=?, avatar_size_pt=?, avatar_y_pt=?, regnum_size=?, regnum_x_pt=?, regnum_y_pt=?, regnum_color=?, nick_size=?, nick_y_pt=?, nick_color=?")
                .bind(page_width_mm)
                .bind(page_height_mm)
                .bind(dpi)
                .bind(avatar_size_pt)
                .bind(avatar_y_pt)
                .bind(regnum_size)
                .bind(regnum_x_pt)
                .bind(regnum_y_pt)
                .bind(regnum_color)
                .bind(nick_size)
                .bind(nick_y_pt)
                .bind(nick_color)
                .execute(&state.db)
                    .await
                    .map_err(AppError::from)?;

                match name_font_file {
                    None => {}
                    Some(f) => {
                        sqlx::query("update layers_config set name_font_path=?")
                            .bind(f)
                            .execute(&state.db)
                            .await
                            .map_err(AppError::from)?;
                    }
                }
                match nr_font_file {
                    None => {}
                    Some(f) => {
                        sqlx::query("update layers_config set nr_font_path=?")
                            .bind(f)
                            .execute(&state.db)
                            .await
                            .map_err(AppError::from)?;
                    }
                }
            }
            _ => {}
        },
        Some(row) => match data.get("action") {
            Some(s) if s == "↑" && row != 1 => {
                let mut tx = state.db.begin().await.unwrap();
                sqlx::query("update layers set id=0 where id=?")
                    .bind(row)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::from)?;
                sqlx::query("update layers set id=? where id=?")
                    .bind(row)
                    .bind(row - 1)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::from)?;
                sqlx::query("update layers set id=? where id=0")
                    .bind(row - 1)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::from)?;
                tx.commit().await.unwrap();
            }
            Some(s) if s == "↓" && row != max_row => {
                let mut tx = state.db.begin().await.unwrap();
                sqlx::query("update layers set id=0 where id=?")
                    .bind(row)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::from)?;
                sqlx::query("update layers set id=? where id=?")
                    .bind(row)
                    .bind(row + 1)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::from)?;
                sqlx::query("update layers set id=? where id=0")
                    .bind(row + 1)
                    .execute(&mut *tx)
                    .await
                    .map_err(AppError::from)?;
                tx.commit().await.unwrap();
            }
            Some(s) if s == "x" => {
                let is_badge =
                    sqlx::query("select id from layers where id=? and asset_path='badge'")
                        .bind(row)
                        .fetch_optional(&state.db)
                        .await
                        .map_err(AppError::from)?
                        .is_some();
                if !is_badge {
                    sqlx::query("delete from layers where id=?")
                        .bind(row)
                        .execute(&state.db)
                        .await
                        .map_err(AppError::from)?;
                    if row != max_row {
                        for n in row..max_row {
                            sqlx::query("update layers set id=? where id=?")
                                .bind(n)
                                .bind(n + 1)
                                .execute(&state.db)
                                .await
                                .map_err(AppError::from)?;
                        }
                    }
                }
            }
            Some(s) if s == "update" => {
                match name_font_file {
                    None => {}
                    Some(f) => {
                        sqlx::query("update layers set asset_path=? where id=?")
                            .bind(f)
                            .bind(row)
                            .execute(&state.db)
                            .await
                            .map_err(AppError::from)?;
                    }
                };
                match nr_font_file {
                    None => {}
                    Some(f) => {
                        sqlx::query("update layers set asset_path=? where id=?")
                            .bind(f)
                            .bind(row)
                            .execute(&state.db)
                            .await
                            .map_err(AppError::from)?;
                    }
                };
                match (
                    data.get("event_is"),
                    data.get("event_name"),
                    data.get("badge_type"),
                    data.get("title"),
                ) {
                    (Some(event_is), Some(event_name), Some(badge_type), Some(title)) => {
                        let event_is =
                            !matches!(event_is.to_lowercase().as_str(), "true" | "t" | "1");
                        let event_name = event_name.to_lowercase();
                        sqlx::query("update layers set event_not=?, event=?, badge_type=?, title=? where id=?")
                            .bind(event_is)
                            .bind(event_name)
                            .bind(badge_type)
                            .bind(title)
                            .bind(row)
                            .execute(&state.db)
                            .await
                            .map_err(AppError::from)?;
                    }

                    _ => return Ok((StatusCode::BAD_REQUEST, "invalid input").into_response()),
                };
            }
            Some(s) if s == "add" => {
                sqlx::query("insert into layers values (?, 'unknown', '', 'any', 0, 'any')")
                    .bind(row)
                    .execute(&state.db)
                    .await
                    .map_err(AppError::from)?;
            }
            Some(s) => {
                warn!(s, "invalid action")
            }
            None => warn!("no action"),
        },
    }

    get_handler(cookies, State(state)).await
}

pub async fn get_handler(
    cookies: CookieJar,
    State(state): State<Arc<types::AppState>>,
    // Form(data): Form<HashMap<String, String>>,
    // mut multipart: Multipart,
    // req: Request
) -> Result<Response, AppError> {
    let (cookies, user) = super::auth::must_be_logged_in(cookies, state.as_ref())?;

    let layers_config =
        sqlx::query_as::<_, types::LayersConfig>("select * from layers_config limit 1;")
            .fetch_one(&state.db)
            .await
            .map_err(AppError::Db)?;

    let layers = sqlx::query_as::<_, types::Layer>("select * from layers order by id asc;")
        .fetch_all(&state.db)
        .await
        .map_err(AppError::from)?;

    let assets_dir = std::env::var("UPLOADS_DIR").unwrap();
    let assets_dir = assets_dir
        .strip_suffix("/")
        .unwrap_or(&assets_dir)
        .to_owned();
    let r = crate::templates::DesignerTemplate {
        auth: user,
        config: layers_config,
        layers,
        assets_dir,
    }
    .render()?;
    return Ok((cookies, Html(r)).into_response());
}
