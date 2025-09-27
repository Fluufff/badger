use std::sync::Arc;

use crate::{
    AppError,
    auth::{self, Claims, LoginResult},
    types::{self, AppState},
};
use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse, Redirect, Response},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{DecodingKey, Validation};
use serde::Deserialize;
use tracing::info;

pub fn must_be_logged_in(
    mut cookies: CookieJar,
    state: &AppState,
) -> Result<(CookieJar, Claims), AppError> {
    let user = cookies.get("auth").map(|c| c.value());

    let user = match user {
        None => None,
        Some(token) => match jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Err(e) => {
                cookies = cookies.remove("auth");
                None
            }
            Ok(data) => Some(data.claims),
        },
    };

    let user = match user {
        Some(user) => user,
        None => return Err(AppError::Unauthorized),
    };
    Ok((cookies, user))
}

pub async fn main_handler(
    cookies: CookieJar,
    State(state): State<Arc<types::AppState>>,
) -> Result<Response, AppError> {
    let (cookies, user) = must_be_logged_in(cookies, state.as_ref())?;

    Ok((cookies, format!("hey {}", user.sub)).into_response())
}

#[derive(Deserialize, Default)]
pub struct LoginPayload {
    user: Option<String>,
    password: Option<String>,
    otp: Option<String>,
    redirect: Option<String>,
}

pub async fn login_handler(
    mut cookies: CookieJar,
    State(state): State<Arc<types::AppState>>,
    Form(login): Form<LoginPayload>,
) -> Result<Response, AppError> {
    info!("handling login");

    let redir = Redirect::temporary(login.redirect.as_deref().unwrap_or("/"));

    if let Some(token) = cookies.get("auth") {
        match jsonwebtoken::decode::<Claims>(
            token.value(),
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Err(e) => {
                info!("can't decode token {:?}", e);
                cookies = cookies.remove("auth");
            }
            Ok(data) => {
                info!("auth {:?}", data);
                return Ok(redir.into_response());
            }
        };
    }

    let mut prefill_user = None;
    let mut prefill_pass = None;
    if let (Some(user), Some(pass)) = (login.user, login.password) {
        match auth::plz_gib_token(state.as_ref(), &user, &pass, &login.otp).await? {
            LoginResult::NoSuchUser => {
                info!("no such user")
            }
            LoginResult::WrongPass => {
                info!("wrong pass");
                prefill_user.replace(user);
            }
            LoginResult::WrongMFA | LoginResult::MissingMFA => {
                info!("shitty mfa");
                prefill_user.replace(user);
                prefill_pass.replace(pass);
            }
            LoginResult::Authenticated(token, _) => {
                info!("authenticated");
                cookies = cookies.add(Cookie::new("auth", token));
                return Ok((cookies, redir).into_response());
            }
        }
    }

    let r = crate::templates::LoginTemplate::new(prefill_user, prefill_pass).render()?;

    Ok((cookies, Html(r)).into_response())
}
