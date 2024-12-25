use actix_web::HttpResponse;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/js"]
struct JSFiles;

pub fn js_file_response(path: &str) -> HttpResponse {
    match JSFiles::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type("application/javascript")
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
