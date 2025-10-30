use askama::Template;

use crate::{
    auth,
    types::{self, Stuff},
};

#[derive(strum::Display, Debug, strum::EnumIs)]
pub enum StuffState {
    No,
    Unpaid,
    Paid,
    Unknown,
}

impl From<&Stuff> for StuffState {
    fn from(input: &Stuff) -> Self {
        match input.paid {
            None => Self::Unknown,
            Some(true) => Self::Paid,
            Some(false) => Self::Unpaid,
        }
    }
}

#[derive(strum::Display, Debug, strum::EnumIs)]
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
    pub ticket_any: BoolState,
    pub ticket_convention: StuffState,
    pub ticket_day: StuffState,
    pub ticket_wed: StuffState,
    pub ticket_thu: StuffState,
    pub ticket_fri: StuffState,
    pub ticket_sat: StuffState,
    pub ticket_sun: StuffState,
    pub sponsor: StuffState,
    pub staff: BoolState,
    pub media: BoolState,
    pub avatar: String,
    pub has_avatar: BoolState,
    pub fursuit_name: Option<String>,
    pub fursuit_species: Option<String>,
    pub fursuit_avatar: String,
    pub has_fursuit_avatar: BoolState,
}

#[derive(Template)]
#[template(path = "main.html")]
pub struct MainTemplate {
    pub auth: auth::Claims,
    pub users: Vec<UserEntry>,
}

#[derive(Template)]
#[template(path = "designer.html")]
pub struct DesignerTemplate {
    pub auth: auth::Claims,

    pub assets_dir: String,
    pub config: types::LayersConfig,
    pub layers: Vec<types::Layer>,
}

#[derive(Template, Default)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub lastuser: Option<String>,
    pub lastpass: Option<String>,
}

#[derive(Template)]
#[template(path = "staff_assign.html")]
pub struct StaffAssignTemplate {
    pub auth: auth::Claims,
    pub users: Vec<UserEntry>,
}
