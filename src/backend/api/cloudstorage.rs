use actix_web::{get, HttpResponse, Responder, web};
use chrono::prelude::*;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, read_dir, read_to_string};
use std::path::Path;
use crate::{
    // loooooooooooooooooooong
    backend::api::structs::cloudstorage::{SystemEntry},
    options::{CUSTOM_CLOUDSTORAGE, CLOUDSTORAGE},
    util::user_path
};

#[get("/fortnite/api/cloudstorage/system")]
pub async fn system() -> impl Responder {
    let mut data = Vec::<SystemEntry>::new();
    
    if CUSTOM_CLOUDSTORAGE == false {
        for (name, content) in CLOUDSTORAGE.iter() {
            let content = content.decrypt();
            
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
    }
    else {
        let dir = [&user_path(), "cloudstorage"].join("\\");
        
        if !Path::new(&dir).is_dir() {
            create_dir_all(dir).unwrap();
            return HttpResponse::Ok().json(data);
        }
        
        for file in read_dir(dir).unwrap() {
            let file = file.unwrap();
            let file_name = file.file_name().into_string().unwrap();
            let file_data = read_to_string(file.path()).unwrap();

            let mut sha1 = Sha1::new();
            let mut sha256 = Sha256::new();

            sha1.update(file_data.as_bytes());
            sha256.update(file_data.as_bytes());

            let sha1 = sha1.finalize();
            let sha256 = sha256.finalize();

            data.push(SystemEntry {
                unique_filename: file_name.clone(),
                filename: file_name,
                hash: format!("{:x}", sha1),
                hash256: format!("{:x}", sha256),
                length: file_data.len(),
                content_type: String::from("application/octet-stream"),
                // spaghetti pretty much
                uploaded: match file.metadata() {
                    Ok(data) => match data.modified() {
                        Ok(time) => {
                            let time: DateTime<Utc> = time.into();
                            time.to_rfc3339_opts(SecondsFormat::Secs, true)
                        }
                        Err(_) => Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
                    },
                    Err(_) => Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
                },
                storage_type: String::from("S3"),
                do_not_cache: false,
            })
        }
    }
    
    HttpResponse::Ok()
        .json(data)
}

#[get("/fortnite/api/cloudstorage/system/config")]
pub async fn system_config() -> impl Responder {
    HttpResponse::NoContent().json(Vec::<String>::new())
}

#[get("/fortnite/api/cloudstorage/system/{file}")]
pub async fn system_file(web::Path(file): web::Path<String>) -> impl Responder {
    let content = if CUSTOM_CLOUDSTORAGE == true {
        read_to_string(
            [&user_path(), "cloudstorage", &file].join("\\")
        ).unwrap()
    } else {
        CLOUDSTORAGE
        .iter()
        .find(|(name, _)| *name == file)
        .unwrap()
        .1
        .decrypt_str().unwrap()
    };
    HttpResponse::Ok()
        .set_header("content-type", "application/octet-stream")
        .body(content)
}