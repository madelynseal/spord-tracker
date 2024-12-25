use std::collections::BTreeMap;

use handlebars::Handlebars;
use serde::Serialize;

lazy_static! {
    static ref HANDLEBARS: Handlebars<'static> = load_templates();
}

pub fn load_templates() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("header", include_str!("../../web/html/header.html"))
        .unwrap();
    handlebars
        .register_template_string("footer", include_str!("../../web/html/footer.html"))
        .unwrap();
    handlebars
        .register_template_string("index", include_str!("../../web/html/index.html"))
        .unwrap();
    handlebars
        .register_template_string("login", include_str!("../../web/html/login.html"))
        .unwrap();

    handlebars
}

pub fn template_index() -> String {
    let header = template_header("Index");
    let footer = template_footer();

    let body = HANDLEBARS.render("index", &()).unwrap();

    format!("{}{}{}", header, body, footer)
}

pub fn template_login() -> String {
    let header = template_header("Login");
    let footer = template_footer();

    let body = HANDLEBARS.render("login", &()).unwrap();

    format!("{}{}{}", header, body, footer)
}

#[derive(Debug, Serialize)]
struct HeaderData {
    pub title: String,
}
pub fn template_header(title: &str) -> String {
    let data = HeaderData {
        title: title.to_owned(),
    };
    HANDLEBARS.render("header", &data).unwrap()
}

pub fn template_footer() -> String {
    //HANDLEBARS.render("footer", &()).unwrap()
    include_str!("../../web/html/footer.html").to_string()
}
