use std::{collections::HashMap, sync::Arc};

use crate::{
    AppError,
    templates::{StuffState, UserEntry},
    types::{self},
};
use askama::Template;
use axum::{
    Form,
    extract::State,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use std::{env, fs};
use tracing::info;
pub mod auth;
pub mod badges;
pub mod fursuits;

pub async fn main_handler(
    cookies: CookieJar,
    State(state): State<Arc<types::AppState>>,
    Form(data): Form<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let (cookies, user) = auth::must_be_logged_in(cookies, state.as_ref())?;

    let users = sqlx::query_as::<_, types::User>("select * from users;")
        .fetch_all(&state.db)
        .await
        .map_err(AppError::Db)?;

    let stuff = sqlx::query_as::<_, types::Stuff>("select rb.regnumber, rooms.type as kind, rooms.name, (rb.oid is null or orders.total_vat=orders.paid) as paid from rooms_booking as rb left join rooms on rb.tid = rooms.tid left join orders on rb.oid=orders.oid order by rb.rbid asc;").fetch_all(&state.db).await.map_err(AppError::Db)?;
    let avatar_dir = env::var("AVATAR_DIR")
        .unwrap_or("/var/www/platyplus/registration.fluufff.org/assets/avatars".into());
    let avatar_files = fs::read_dir(&avatar_dir)
        .ok()
        .into_iter()
        .flat_map(|entries| entries)
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let p = entry.path();
            if p.is_file() {
                Some(p.file_name()?.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let users = users
        .into_iter()
        .map(|u| {
            let ticket = stuff
                .iter()
                .find(|stuff| stuff.regnumber == u.regnumber && stuff.kind == "Ticket");
            let ticket = match ticket {
                None => StuffState::No,
                Some(ticket) => match ticket.paid {
                    None => StuffState::Unknown,
                    Some(true) => StuffState::Paid,
                    Some(false) => StuffState::Unpaid,
                },
            };

            let sponsor = stuff
                .iter()
                .find(|stuff| stuff.regnumber == u.regnumber && stuff.name == "Sponsor Pack");
            let sponsor = match sponsor {
                None => StuffState::No,
                Some(sponsor) => match sponsor.paid {
                    None => StuffState::Unknown,
                    Some(true) => StuffState::Paid,
                    Some(false) => StuffState::Unpaid,
                },
            };

            let staff = stuff
                .iter()
                .find(|stuff| stuff.regnumber == u.regnumber && stuff.name == "Staff")
                .is_some()
                .into();

            let avatar = avatar_files
                .iter()
                .find(|path| path.contains(&format!("full_{}.", &u.regnumber)))
                // .map(|p| p.into());
                .map(|p| format!("{}/{}", avatar_dir, p));
            let has_avatar = avatar.is_some().into();
            let avatar = avatar.unwrap_or(format!("{}/{}", avatar_dir, "full.png"));

            UserEntry {
                regnumber: u.regnumber,
                nickname: u.nick.unwrap_or_default(),
                sponsor,
                ticket,
                staff,
                avatar,
                has_avatar,
            }
        })
        .collect::<Vec<_>>();

    let mut print_ids = data
        .into_iter()
        .filter(|(_, v)| v == "on")
        .map(|(k, _)| k)
        .filter_map(|k| str::parse(&k).ok())
        .collect::<Vec<i32>>();
    print_ids.sort();
    info!("print IDs: {:?}", print_ids);

    if print_ids.len() == 0 {
        let r = crate::templates::MainTemplate { users, auth: user }.render()?;
        return Ok((cookies, Html(r)).into_response());
    }

    let mut doc = badges::BadgePDF::init(&state.db, "Fluufff badges").await?;

    for reg_id in print_ids {
        let user = users.iter().find(|u| u.regnumber == reg_id).unwrap();
        doc.add_user(user).unwrap();
    }

    let pdf_bytes = doc.print();

    let resp = (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/pdf")],
        pdf_bytes,
    )
        .into_response();

    Ok(resp)
}
