use super::user_logged_in;
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/")]
pub async fn index(id: Option<Identity>) -> impl Responder {
    if let Some(_username) = user_logged_in(id) {
        super::files::html_file_response("index.html")
    } else {
        HttpResponse::Found()
            .insert_header(("location", "/login?redirect=/"))
            .finish()
    }
}

#[get("/js/{path}")]
pub async fn js_file(path: web::Path<String>, id: Option<Identity>) -> HttpResponse {
    let path = path.into_inner();
    if path == String::from("login.js") {
        return super::files::js_file_response(&path);
    }
    if let Some(_username) = user_logged_in(id) {
        super::files::js_file_response(&path)
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginHtml {
    pub redirect: Option<String>,
}
#[get("/login")]
pub async fn login(id: Option<Identity>, params: web::Query<LoginHtml>) -> impl Responder {
    if let Some(_username) = user_logged_in(id) {
        let redirect = if let Some(redirect) = &params.redirect {
            if redirect.starts_with("/") {
                redirect
            } else {
                "/"
            }
        } else {
            "/"
        };
        HttpResponse::Ok()
            .insert_header(("location", redirect))
            .finish()
    } else {
        super::files::html_file_response("login.html")
    }
}
