use printpdf::{Mm, Pt};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::templates::UserEntry;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    // pub assets_dir: PathBuf,
    pub jwt_secret: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub regnumber: i32,
    // pub email: String,
    pub hash: String,
    pub nick: Option<String>,
    // pub gid: i32,
    // pub double_auth: Option<i32>,
    pub double_auth_secret: Option<String>,
    // pub locked: Option<String>,
    pub pronouns: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct StaffAssignments {
    pub regnumber: i32,
    pub media: bool,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Layer {
    pub id: i32,
    pub title: String,
    pub asset_path: String,

    pub event: LayerOpt,
    pub event_not: bool,
    pub event2: LayerOpt,
    pub event2_not: bool,
    pub badge_type: BadgeOpt,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type, strum::EnumIs)]
#[sqlx(rename_all = "snake_case")]
pub enum LayerOpt {
    Any,
    Fursuit,
    Sponsor,
    Sponsor1,
    Sponsor2,
    Sponsor3,
    Staff,
    Media,
    Ticket,
    TicketConvention,
    TicketDay,
    TicketWed,
    TicketThu,
    TicketFri,
    TicketSat,
    TicketWeekend,
}

impl LayerOpt {
    pub fn should(&self, user: &UserEntry, should_not: bool, add_media: bool) -> bool {
        match self {
            LayerOpt::Any => !should_not,
            LayerOpt::Fursuit => false,
            LayerOpt::Staff => user.staff.is_no() == should_not,
            LayerOpt::Media => add_media,
            LayerOpt::Sponsor => user.sponsor.is_no() == should_not,
            LayerOpt::Sponsor1 => user.sponsor_1.is_no() == should_not,
            LayerOpt::Sponsor2 => user.sponsor_2.is_no() == should_not,
            LayerOpt::Sponsor3 => user.sponsor_3.is_no() == should_not,
            LayerOpt::Ticket => user.ticket_any.is_no() == should_not,
            LayerOpt::TicketConvention => user.ticket_convention.is_no() == should_not,
            LayerOpt::TicketDay => user.ticket_day.is_no() == should_not,
            LayerOpt::TicketWed => user.ticket_wed.is_no() == should_not,
            LayerOpt::TicketThu => user.ticket_thu.is_no() == should_not,
            LayerOpt::TicketFri => user.ticket_fri.is_no() == should_not,
            LayerOpt::TicketSat => user.ticket_sat.is_no() == should_not,
            LayerOpt::TicketWeekend => user.ticket_weekend.is_no() == should_not,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type, strum::EnumIs)]
#[sqlx(rename_all = "snake_case")]
pub enum BadgeOpt {
    Any,
    Fursuit,
    ConTicket,
    DayTicket,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Stuff {
    pub regnumber: i32,
    pub kind: String,
    pub name: String,
    pub paid: Option<bool>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct LayersConfig {
    pub page_width_mm: f32,
    pub page_height_mm: f32,

    pub dpi: f32,

    pub avatar_size_pt: f32,
    pub avatar_y_pt: f32,

    pub name_font_path: String,
    pub nr_font_path: String,

    pub regnum_size: f32,
    pub regnum_x_pt: f32,
    pub regnum_y_pt: f32,
    pub regnum_color: String,

    pub nick_size: f32,
    pub nick_y_pt: f32,
    pub nick_color: String,
}

impl LayersConfig {
    pub fn page_width(&self) -> Mm {
        Mm(self.page_width_mm)
    }
    pub fn page_height(&self) -> Mm {
        Mm(self.page_height_mm)
    }

    pub fn avatar_size(&self) -> Pt {
        Pt(self.avatar_size_pt)
    }
    pub fn avatar_y(&self) -> Pt {
        Pt(self.avatar_y_pt)
    }

    pub fn regnum_size(&self) -> Pt {
        Pt(self.regnum_size)
    }
    #[allow(dead_code)]
    pub fn regnum_x(&self) -> Pt {
        Pt(self.regnum_x_pt)
    }
    pub fn regnum_y(&self) -> Pt {
        Pt(self.regnum_y_pt)
    }

    pub fn nick_size(&self) -> Pt {
        Pt(self.nick_size)
    }
    pub fn nick_y(&self) -> Pt {
        Pt(self.nick_y_pt)
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct FursuitAnswer {
    pub regnumber: i32,
    pub name: String,
    pub value: String,
}
