use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};

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
async fn items(
    items: web::Data<Vec<CItem>>
) -> impl Responder {
    HttpResponse::Ok()
        .json(&**items)
}


#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let cosmetics = web::Data::new(
        if COSMETICS == true {
            cosmetics::get()
            .unwrap()
        } else {
            Vec::new()
        }
    );
    
    HttpServer::new(move ||
        App::new()
            .app_data(cosmetics.clone())
            .service(index)
            .service(items)
            .service(api::cloudstorage::system)
            .service(api::cloudstorage::system_config)
            .service(api::cloudstorage::system_file)
        )
        .bind(HOST_URL)?
        .run()
        .await
}
