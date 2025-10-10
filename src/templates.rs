use askama::Template;

use crate::auth;

#[derive(strum::Display, Debug)]
pub enum StuffState {
    No,
    Unpaid,
    Paid,
    Unknown,
}

#[derive(strum::Display, Debug)]
pub enum BoolState {
    No,
    Yes,
}

impl From<bool> for BoolState {
    fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
    }
}

pub struct UserEntry {
    pub regnumber: i32,
    pub nickname: String,
    pub ticket: StuffState,
    pub sponsor: StuffState,
    pub staff: BoolState,
    pub avatar: String,
    pub has_avatar: BoolState,
}

#[derive(Template)]
#[template(path = "main.html")]
pub struct MainTemplate {
    pub auth: auth::Claims,
    pub users: Vec<UserEntry>,
}

#[derive(Template)]
#[template(path = "layers.html")]
pub struct LayersTemplate {}

#[derive(Template, Default)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub lastuser: Option<String>,
    pub lastpass: Option<String>,
}
