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
use sqlx::{MySql, Pool};
use std::{env, fs};
use tracing::info;
pub mod auth;
pub mod badges;
pub mod designer;
pub mod staff_assign;

pub async fn main_handler(
    cookies: CookieJar,
    State(state): State<Arc<types::AppState>>,
    Form(data): Form<HashMap<String, String>>,
) -> Result<Response, AppError> {
    let (cookies, user) = auth::must_be_logged_in(cookies, state.as_ref())?;

    let users = get_users(&state.db).await?;

    if data.len() == 0 {
        let r = crate::templates::MainTemplate { users, auth: user }.render()?;
        return Ok((cookies, Html(r)).into_response());
    }

    let total = data.len();
    info!(total, "printing badges");

    let mut doc = badges::BadgePDF::init(&state.db, "Fluufff badges").await?;

    let mut count_done = 0;
    for user in users {
        if let Some(v) = data.get(&format!("user_{}", &user.regnumber))
            && v == "on"
        {
            info!(id = &user.regnumber, "{count_done}/{total} printing user");
            doc.add_user(&user, false, false).unwrap();
            if user.medic.is_yes() {
                doc.add_user(&user, true, false).unwrap();
            }
            if user.security.is_yes() {
                doc.add_user(&user, false, true).unwrap();
            }
            count_done += 1;
        }
        if let Some(v) = data.get(&format!("fursuit_{}", &user.regnumber))
            && v == "on"
        {
            info!(
                id = &user.regnumber,
                "{count_done}/{total} printing fursuit"
            );
            doc.add_fursuit(&user).unwrap();
            count_done += 1;
        }
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

pub async fn get_users(db: &Pool<MySql>) -> Result<Vec<UserEntry>, AppError> {
    let users = sqlx::query_as::<_, types::User>("select * from users;")
        .fetch_all(db)
        .await
        .map_err(AppError::Db)?;

    let staff_assignments =
        sqlx::query_as::<_, types::StaffAssignments>("select * from staff_assignments;")
            .fetch_all(db)
            .await
            .map_err(AppError::Db)?;

    let fursuit_entries = sqlx::query_as::<_, types::FursuitAnswer>("select fv.regnumber, f.name, fv.value from registrations_forms as f left join registrations_forms_list as fl on fl.fid=f.fid left join registrations_forms_values as fv on fv.ffid=f.ffid where fl.name='Fursuiter' and fv.value!='';").fetch_all(db).await.map_err(AppError::Db)?;

    let stuff = sqlx::query_as::<_, types::Stuff>("select rb.regnumber, rooms.type as kind, rooms.name, (rb.oid is null or orders.total_vat=orders.paid) as paid from rooms_booking as rb left join rooms on rb.tid = rooms.tid left join orders on rb.oid=orders.oid order by rb.rbid asc;").fetch_all(db).await.map_err(AppError::Db)?;
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

    let uploads_dir = env::var("UPLOADS_DIR")
        .unwrap_or("/var/www/platyplus/registration.fluufff.org/uploads".into());
    let fursuit_files = fs::read_dir(&uploads_dir)
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
                .map(|p| format!("{}/{}", avatar_dir, p));
            let has_avatar = avatar.is_some().into();
            let avatar = avatar.unwrap_or(format!("{}/{}", avatar_dir, "full.png"));

            let fursuit_name = fursuit_entries
                .iter()
                .find(|fe| &fe.regnumber == &u.regnumber && fe.name == "Name")
                .map(|fe| fe.value.clone());
            let fursuit_species = fursuit_entries
                .iter()
                .find(|fe| &fe.regnumber == &u.regnumber && fe.name == "Species")
                .map(|fe| fe.value.clone());

            let fursuit_avatar = fursuit_files
                .iter()
                .find(|path| path.contains(&format!("up_{}.", &u.regnumber)))
                .map(|p| format!("{}/{}", uploads_dir, p));
            let has_fursuit_avatar = fursuit_avatar.is_some().into();
            let fursuit_avatar = fursuit_avatar.unwrap_or(format!("{}/{}", avatar_dir, "full.png"));

            let medic = staff_assignments
                .iter()
                .any(|ass| ass.regnumber == u.regnumber && ass.medic)
                .into();
            let security = staff_assignments
                .iter()
                .any(|ass| ass.regnumber == u.regnumber && ass.security)
                .into();

            UserEntry {
                regnumber: u.regnumber,
                nickname: u.nick.unwrap_or_default(),
                sponsor,
                ticket,
                staff,
                medic,
                security,
                avatar,
                has_avatar,
                fursuit_name,
                fursuit_species,
                fursuit_avatar,
                has_fursuit_avatar,
            }
        })
        .collect();

    Ok(users)
}
