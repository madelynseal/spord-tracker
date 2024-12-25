use super::template;
use super::user_logged_in;
use crate::sql;
use actix_identity::Identity;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/")]
pub async fn index(id: Option<Identity>) -> impl Responder {
    if let Some(_username) = user_logged_in(id) {
        let html = template::template_index();

        HttpResponse::Ok().body(html)
    } else {
        HttpResponse::Found()
            .insert_header(("location", "/login"))
            .finish()
    }
}

#[get("/js/{path}")]
pub async fn js_file(path: web::Path<String>, id: Option<Identity>) -> HttpResponse {
    if let Some(_username) = user_logged_in(id) {
        super::files::js_file_response(&path)
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[get("/login")]
pub async fn login(id: Option<Identity>) -> impl Responder {
    if let Some(_username) = user_logged_in(id) {
        HttpResponse::Ok().insert_header(("location", "/")).finish()
    } else {
        HttpResponse::Ok().body(template::template_login())
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginPostData {
    pub username: String,
    pub password: String,
}
#[post("/login_post")]
pub async fn login_post(
    request: HttpRequest,
    id: Option<Identity>,
    params: web::Form<LoginPostData>,
) -> actix_web::Result<HttpResponse> {
    if let Some(_username) = user_logged_in(id) {
        Ok(HttpResponse::Ok().insert_header(("location", "/")).finish())
    } else {
        if sql::user_login(None, &params.username, &params.password)
            .await
            .unwrap()
        {
            Identity::login(&request.extensions(), format!("user:{}", &params.username))?;

            Ok(HttpResponse::Found()
                .insert_header(("location", "/"))
                .finish())
        } else {
            Ok(HttpResponse::Found()
                .insert_header(("location", "/login"))
                .finish())
        }
    }
}

#[get("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.logout();

    HttpResponse::Found()
        .insert_header(("location", "/login"))
        .finish()
}
