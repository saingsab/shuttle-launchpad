use core::fmt;
use std::fmt::Display;
use std::{io::Cursor, fmt::Formatter};

use axum::http::StatusCode;
use axum::{routing::post, Router, response::IntoResponse};
use bytes::Bytes;
use image::ImageOutputFormat;

#[derive(Debug)]
struct ProcessImageError(String);

impl Display for ProcessImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ProcessImageError: {}", self.0)
    }
}

impl std::error::Error for ProcessImageError {}

impl From<image::ImageError> for ProcessImageError {
    fn from(err: image::ImageError) -> Self {
        ProcessImageError(format!("ImageError: {}", err))
    }
}

impl IntoResponse for ProcessImageError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

struct ImageResponse(Bytes);

impl IntoResponse for ImageResponse {
    fn into_response(self) -> axum::response::Response {
        ([("content-type", "image/png")], self.0).into_response()
    }
}

async fn process_image(bytes: Bytes) -> Result<ImageResponse, ProcessImageError> {
    let image = image::load_from_memory(&bytes)?;

    let mut vec: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut vec);

    image
        .grayscale()
        .write_to(&mut cursor, ImageOutputFormat::Png)?;

    let bytes: Bytes = vec.into();

    Ok(ImageResponse(bytes))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", post(process_image));

    Ok(router.into())
}
