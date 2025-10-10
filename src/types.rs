use printpdf::{Mm, Pt};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    // pub assets_dir: PathBuf,
    pub jwt_secret: String,
}

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub badge_ids: Vec<i64>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub regnumber: i32,
    // pub email: String,
    pub hash: String,
    pub nick: Option<String>,
    pub gid: i32,
    // pub double_auth: Option<i32>,
    pub double_auth_secret: Option<String>,
    // pub locked: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Layer {
    pub id: i32,
    pub title: String,
    pub asset_path: String,

    pub on_sponsor: bool,
    pub on_staff: bool,
    pub on_staff_security: bool,
    pub on_staff_medic: bool,
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
    page_width_mm: f32,
    page_height_mm: f32,

    pub dpi: f32,

    avatar_size_pt: f32,
    avatar_y_pt: f32,

    pub font_path: String,

    regnum_size: f32,
    regnum_x_pt: f32,
    regnum_y_pt: f32,

    nick_size: f32,
    nick_y_pt: f32,
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
