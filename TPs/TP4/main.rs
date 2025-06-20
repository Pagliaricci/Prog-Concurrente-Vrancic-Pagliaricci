use actix_multipart::Multipart;
use actix_web::{
    web, App, Error, HttpResponse, HttpServer, Responder, get, post, http::StatusCode,
};
use futures_util::stream::StreamExt as _;
use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}},
};
use serde::Serialize;
use lazy_static::lazy_static;

lazy_static! {
    static ref STATS: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref ACTIVE_UPLOADS: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
}

const MAX_CONCURRENT_UPLOADS: usize = 4;

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    if ACTIVE_UPLOADS.load(Ordering::SeqCst) >= MAX_CONCURRENT_UPLOADS {
        return Ok(HttpResponse::TooManyRequests().body("Server is busy. Try again later."));
    }

    ACTIVE_UPLOADS.fetch_add(1, Ordering::SeqCst);

    let mut total_exceptions = 0;
    let mut found_file = false;

    while let Some(field) = payload.next().await {
        let mut field = field?;
        let content_disposition = field.content_disposition();

        if let Some(filename) = content_disposition.get_filename() {
            found_file = true;

            let mut content = Vec::new();
            while let Some(chunk) = field.next().await {
                content.extend_from_slice(&chunk?);
            }

            if content.is_empty() {
                ACTIVE_UPLOADS.fetch_sub(1, Ordering::SeqCst);
                return Ok(HttpResponse::BadRequest().body("Uploaded file is empty."));
            }

            let text = String::from_utf8_lossy(&content);
            total_exceptions = text.lines().filter(|line| line.contains("exception")).count();

            // Guardar stats
            let mut stats = STATS.lock().unwrap();
            stats.insert(filename.to_string(), total_exceptions);
        }
    }

    ACTIVE_UPLOADS.fetch_sub(1, Ordering::SeqCst);

    if !found_file {
        return Ok(HttpResponse::BadRequest().body("No file uploaded."));
    }

    Ok(HttpResponse::Ok().json(
        serde_json::json!({ "exceptions": total_exceptions })
    ))
}

#[derive(Serialize)]
struct StatsResponse {
    archivos: HashMap<String, usize>,
}

#[get("/stats")]
async fn stats() -> impl Responder {
    let stats = STATS.lock().unwrap();
    let data = stats.clone();

    HttpResponse::Ok().json(StatsResponse {
        archivos: data,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor corriendo en http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(upload)
            .service(stats)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
