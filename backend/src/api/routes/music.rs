use crate::api::services::{
    jwt::{get_token, verify_jwt},
    loader::{AppState, MusicInfo},
};
use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header, web};
use std::io::SeekFrom;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};

#[get("/")]
pub async fn list(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let token = match get_token(req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().body("Missing or invalid token"),
    };
    match verify_jwt(&token) {
        Ok(_) => {
            let tracks = data.tracks.lock().unwrap();
            let track_list: Vec<&MusicInfo> = tracks.values().collect();
            HttpResponse::Ok().json(track_list)
        }
        Err(e) => {
            eprintln!("JWT Error: {e:?}");
            HttpResponse::Unauthorized().body("Invalid token")
        }
    }
}

#[get("/{path:.*}")]
#[allow(clippy::manual_strip, clippy::await_holding_lock)]
pub async fn stream(
    path: web::Path<String>,
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let token = match get_token(req.clone()) {
        Some(t) => t,
        None => return Ok(HttpResponse::Unauthorized().body("Missing or invalid token")),
    };
    if let Err(e) = verify_jwt(&token) {
        eprintln!("JWT Error: {e:?}");
        return Ok(HttpResponse::Unauthorized().body("Invalid token"));
    }
    let req_path = path.into_inner();
    let tracks = data.tracks.lock().unwrap();

    let track_info = tracks
        .get(&req_path)
        .ok_or_else(|| actix_web::error::ErrorNotFound(format!("Track not found: {req_path}")))?;
    let file_path = data.music_dir.join(&track_info.path);
    let file_size;
    let mut file = match File::open(&file_path).await {
        Ok(f) => {
            file_size = f.metadata().await?.len();
            f
        }
        Err(e) => {
            eprintln!("Failed to open file {}: {}", file_path.display(), e);
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to open file",
            ));
        }
    };

    let range_header = req.headers().get(header::RANGE);

    if let Some(range_header_value) = range_header {
        let range_str = range_header_value
            .to_str()
            .map_err(|_| actix_web::error::ErrorBadRequest("Invalid Range header"))?;

        if range_str.starts_with("bytes=") {
            let parts: Vec<&str> = range_str["bytes=".len()..].split('-').collect();
            if parts.len() == 2 {
                let start_byte: u64 = parts[0].parse().unwrap_or(0);
                let end_byte_option: Option<u64> = parts[1].parse().ok();

                let end_byte = end_byte_option.unwrap_or(file_size - 1);

                if start_byte >= file_size || start_byte > end_byte {
                    return Ok(HttpResponse::RangeNotSatisfiable()
                        .insert_header(header::ContentRange(
                            format!("bytes */{file_size}").parse().unwrap(),
                        ))
                        .finish());
                }

                file.seek(SeekFrom::Start(start_byte)).await?;

                let content_length = (end_byte - start_byte + 1).min(file_size - start_byte);
                let content_range = format!(
                    "bytes {}-{}/{}",
                    start_byte,
                    start_byte + content_length - 1,
                    file_size
                );

                let stream = tokio_util::io::ReaderStream::new(file.take(content_length));

                return Ok(HttpResponse::PartialContent()
                    .insert_header(header::ContentType(track_info.mime.parse().unwrap()))
                    .insert_header(header::ContentLength(content_length as usize))
                    .insert_header(header::ContentRange(content_range.parse().unwrap()))
                    .streaming(stream));
            }
        }
    }

    let stream = tokio_util::io::ReaderStream::new(file);

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(track_info.mime.parse().unwrap()))
        .insert_header(header::ContentLength(file_size as usize))
        .streaming(stream))
}
