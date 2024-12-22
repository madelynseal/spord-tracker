use super::user_logged_in;
use crate::sql;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AuthLogin {
    username: String,
    password: String,
}
#[derive(Debug, Deserialize)]
struct AuthLoginQuery {
    redirect: Option<String>,
}
#[post("/api/user/login")]
async fn api_user_login(
    request: HttpRequest,
    params: web::Form<AuthLogin>,
    query: web::Query<AuthLoginQuery>,
) -> actix_web::Result<HttpResponse> {
    //TODO: process error properly
    if sql::user_login(None, &params.username, &params.password)
        .await
        .unwrap()
    {
        Identity::login(&request.extensions(), format!("user:{}", &params.username))?;

        let redirect = if let Some(redirect) = &query.redirect {
            if redirect.starts_with('/') {
                redirect
            } else {
                "/"
            }
        } else {
            "/"
        };

        Ok(HttpResponse::Found()
            .insert_header(("location", redirect))
            .finish())
    } else {
        Ok(HttpResponse::Found()
            .insert_header(("location", "/login"))
            .finish())
    }
}
