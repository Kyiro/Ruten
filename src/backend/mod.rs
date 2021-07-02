use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::backend::cosmetics::CItem;
use crate::options::{COSMETICS, HOST_URL};

mod api;
mod cosmetics;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header("Location", "https://github.com/Kyiro")
        .finish()
}

#[get("/cosmetics")]
async fn items(items: web::Data<Vec<CItem>>) -> impl Responder {
    HttpResponse::Ok().json(&**items)
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let cosmetics = web::Data::new(if COSMETICS == true {
        cosmetics::get().unwrap()
    } else {
        Vec::new()
    });

    HttpServer::new(move || {
        App::new()
            .app_data(cosmetics.clone())
            .service(index)
            .service(items)
            .service(api::cloudstorage::system)
            .service(api::cloudstorage::system_config)
            .service(api::cloudstorage::system_file)
            .service(api::mcp::client_quest_login)
            .service(api::mcp::set_cosmetic_locker_slot)
            .service(api::mcp::set_item_favorite_status_batch)
            .service(api::mcp::query_profile)
            .service(api::mcp::other)
    })
    .bind(HOST_URL)?
    .run()
    .await
}
