//! This server is responsible for managing the tagging of objects and the creation of records
//! for objects in the system.
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use rust_embed::RustEmbed;
use std::path::Path;
use actix_cors::Cors;
use auth_networking::api::views_factory as auth_views_factory;
use to_do_networking::api::views_factory as to_do_views_factory;
use dal::migrations::run_migrations;
use actix_web::middleware::Logger;


/// Serves the HTML file for the frontend which will load the bundle.js file. 
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(include_str!("../../frontends/web/public/index.html"))
}


/// Catches all requests that are not handled by the other routes. If the route does not have "/api/" in it, then
/// it will check to see if the request is for static files from the frontend or admin frontend. If it is, then it will
/// serve the file. Otherwise, it will serve the index.html file for the frontend or the index_admin.html file for the
/// admin frontend.
/// 
/// # Arguments
/// * `req` - The request that is being handled.
/// 
/// # Returns
/// bytes of a file
async fn catch_all(req: HttpRequest) -> impl Responder {
    if req.path().contains("/api/") {
        return HttpResponse::NotFound().finish()
    }
    if req.path().contains("frontend/public") {
        return serve_frontend_asset(req.path().to_string())
    }
    let file_type = match mime_guess::from_path(&req.path()).first_raw() {
        Some(file_type) => file_type,
        None => "text/html"
    };
    if !file_type.contains("text/html") {
        return serve_frontend_asset(req.path().to_string())
    }
    index().await
}


/// Embeds the frontend files into the binary.
#[derive(RustEmbed)]
#[folder = "../frontends/web/public"]
struct FrontendAssets;


/// Serves the frontend files from the binary.
/// 
/// # Arguments
/// * `path` - The path from the request.
/// 
/// # Returns
/// a http response with the bytes of the file
fn serve_frontend_asset(path: String) -> HttpResponse {
    let file = match Path::new(&path).file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return HttpResponse::BadRequest().body("404 Not Found")
    };
    match FrontendAssets::get(file) {
        Some(content) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(&file).first_or_octet_stream().as_ref())
            .append_header(("Cache-Control", "public, max-age=604800"))
            .body(content.data),
        None => HttpResponse::NotFound().body("404 Not Found")
    }
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

    // init_logger();
    run_migrations().await;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        App::new()
            .configure(auth_views_factory)
            .configure(to_do_views_factory)
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
            .default_service(web::route().to(catch_all))
    })
        .bind("0.0.0.0:8001")?
        .run()
        .await
}