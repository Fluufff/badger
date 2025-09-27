use askama::Template;
pub struct GenericTemplate {
    pub name: &'static str,
}

impl Default for GenericTemplate {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
        }
    }
}

#[derive(Template)]
#[template(path = "layers.html")]
pub struct LayersTemplate {}

#[derive(Template, Default)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub generic: GenericTemplate,
    pub lastuser: Option<String>,
    pub lastpass: Option<String>,
}

impl LoginTemplate {
    pub fn new(lastuser: Option<String>, lastpass: Option<String>) -> Self {
        Self {
            generic: Default::default(),
            lastuser,
            lastpass,
        }
    }
}
