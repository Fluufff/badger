use serde::{Deserialize, Serialize};

use crate::{AppError, types};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String, // nickname
    pub exp: usize,
}

pub enum LoginResult {
    NoSuchUser,
    WrongPass,
    MissingMFA,
    WrongMFA,
    NotStaff,
    Authenticated(String, Claims),
}

pub async fn plz_gib_token(
    state: &types::AppState,
    user: &str,
    pass: &str,
    otp: &Option<String>,
) -> Result<LoginResult, AppError> {
    let row =
        sqlx::query_as::<_, types::User>("SELECT * FROM users WHERE email = ? or nick = ? LIMIT 1")
            .bind(user)
            .bind(user)
            .fetch_optional(&state.db)
            .await
            .map_err(AppError::Db)?;

    let row = match row {
        None => return Ok(LoginResult::NoSuchUser),
        Some(r) => r,
    };

    if !bcrypt::verify(pass, &row.hash)? {
        return Ok(LoginResult::WrongPass);
    }

    match (row.double_auth_secret, otp) {
        (None, _) => {}
        (Some(_), None) => return Ok(LoginResult::MissingMFA),
        (Some(secret), Some(otp)) => {
            let (valid, _) = thotp::verify_totp(&otp, secret.as_bytes(), 0)?;
            if !valid {
                return Ok(LoginResult::WrongMFA);
            }
        }
    }

    let staff_row = sqlx::query("select users.nick, rooms.name from rooms_booking as rb left join rooms on rb.tid = rooms.tid left join users on users.regnumber=rb.regnumber where rooms.name='Staff' and users.email = ? or users.nick = ? order by rb.rbid asc;")
            .bind(user)
            .bind(user).fetch_optional(&state.db).await
            .map_err(AppError::Db)?;

    if staff_row.is_none() {
        return Ok(LoginResult::NotStaff);
    }

    let exp = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: row.nick.unwrap_or_else(|| format!("#{}", row.regnumber)),
        exp,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )?;
    Ok(LoginResult::Authenticated(token, claims))
}
