use crate::{
    // loooooooooooooooooooong
    backend::api::structs::cloudstorage::SystemEntry,
    options::{CLOUDSTORAGE, CUSTOM_CLOUDSTORAGE},
    util::user_path,
};
use actix_web::{get, web, HttpResponse, Responder};
use chrono::prelude::*;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, read_dir, read_to_string};
use std::path::Path;

#[get("/fortnite/api/cloudstorage/system")]
pub async fn system() -> std::io::Result<impl Responder> {
    let mut data = Vec::<SystemEntry>::new();
    let cloudstorage: Vec<(_, _)> = if CUSTOM_CLOUDSTORAGE == true {
        let dir = [&user_path(), "cloudstorage"].join("\\");

        if !Path::new(&dir).is_dir() {
            create_dir_all(dir).unwrap();
            return Ok(HttpResponse::Ok().json(data));
        }
        
        read_dir(dir)?
        .into_iter()
        .map(|f| {
            let file = f.unwrap();
            
            (
                file.file_name().into_string().unwrap(),
                read_to_string(file.path()).unwrap()
            )
        })
        .collect()
    } else {
        CLOUDSTORAGE
        .iter()
        .map(|(n, c)| (n.to_string(), c.decrypt_str().unwrap()))
        .collect()
    };

    for (name, content) in cloudstorage.into_iter() {
        // spaghetti :flushed:
        let content = String::into_bytes(content);
        
        let mut sha1 = Sha1::new();
        let mut sha256 = Sha256::new();
        sha1.update(&content);
        sha256.update(&content);
        let sha1 = sha1.finalize();
        let sha256 = sha256.finalize();

        let content = String::from_utf8(content).unwrap();

        data.push(SystemEntry {
            unique_filename: name.to_string(),
            filename: name.to_string(),
            hash: format!("{:x}", sha1),
            hash256: format!("{:x}", sha256),
            length: content.len(),
            content_type: String::from("application/octet-stream"),
            uploaded: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            storage_type: String::from("S3"),
            do_not_cache: true,
        })
    }

    Ok(HttpResponse::Ok().json(data))
}

#[get("/fortnite/api/cloudstorage/system/config")]
pub async fn system_config() -> impl Responder {
    HttpResponse::NoContent().json(Vec::<String>::new())
}

#[get("/fortnite/api/cloudstorage/system/{file}")]
pub async fn system_file(web::Path(file): web::Path<String>) -> impl Responder {
    let content = if CUSTOM_CLOUDSTORAGE == true {
        read_to_string([&user_path(), "cloudstorage", &file].join("\\")).unwrap()
    } else {
        CLOUDSTORAGE
            .iter()
            .find(|(name, _)| *name == file)
            .unwrap()
            .1
            .decrypt_str()
            .unwrap()
    };
    HttpResponse::Ok()
        .set_header("content-type", "application/octet-stream")
        .body(content)
}
