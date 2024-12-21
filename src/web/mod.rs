use crate::{constants, sql, CONFIG};
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Cookie;
use actix_web::{cookie::Key, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::BufReader;

mod files;
mod html;
mod user;

pub async fn start() -> std::io::Result<()> {
    let secret_key = Key::generate();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
    });

    let listen_address = &CONFIG.web.listen;
    let https = CONFIG.web.https.unwrap_or(false);
    info!("Will listen on {} https: {}", listen_address, https);

    if https {
        unimplemented!("actix-web https");
    } else {
        server.bind(listen_address)?.run().await?;
    }

    Ok(())
}
