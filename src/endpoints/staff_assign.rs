use std::{collections::HashMap, sync::Arc};

use crate::{
    AppError,
    types::{self},
};
use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse, Response},
};
use axum_extra::extract::CookieJar;

pub async fn handler(
    cookies: CookieJar,
    State(state): State<Arc<types::AppState>>,
    Form(data): Form<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let (cookies, user) = super::auth::must_be_logged_in(cookies, state.as_ref())?;

    match data
        .get("regnumber")
        .and_then(|s| str::parse::<i32>(s).ok())
    {
        None => {}
        Some(regnumber) => {
            let medic = data.contains_key("medic");
            let security = data.contains_key("security");
            let exists = sqlx::query("select regnumber from staff_assignments where regnumber=?")
                .bind(regnumber)
                .fetch_optional(&state.db)
                .await
                .map_err(AppError::from)?;
            if exists.is_some() {
                sqlx::query("update staff_assignments set medic=?, security=? where regnumber=?")
                    .bind(medic)
                    .bind(security)
                    .bind(regnumber)
                    .execute(&state.db)
                    .await
                    .map_err(AppError::from)?;
            } else {
                sqlx::query("insert into staff_assignments values (?, ?, ?)")
                    .bind(regnumber)
                    .bind(medic)
                    .bind(security)
                    .execute(&state.db)
                    .await
                    .map_err(AppError::from)?;
            }
        }
    }

    let users = super::get_users(&state.db).await?;
    let staff = users.into_iter().filter(|u| u.staff.is_yes()).collect();

    let r = crate::templates::StaffAssignTemplate { staff, auth: user }.render()?;
    return Ok((cookies, Html(r)).into_response());
}
