use anyhow::Context;
// use core::slice::SlicePattern;
use graphicsmagick::{
    initialize, types::FilterTypes, types::GravityType, wand::MagickWand, MagickBoxSlice,
};
use std::path::PathBuf;

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::{StatusCode, header},
    response::{Html, Response, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

fn make_square(filename: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("yeet.jpg");
    let path = path.to_str().context("get image path failed")?;

    initialize();

    let mut binding = MagickWand::new();
    let mut mw = binding
        .read_image(path)?
        .set_image_gravity(graphicsmagick::types::GravityType::CenterGravity)?
        .transform_image("", "2000x2000>")
        .expect("yeet");

    if mw.get_image_height() == 2000 {
        let x = &mw.get_image_width() / 2;
        mw.extent_image(2200, 2200, (1100 - x).try_into().unwrap(), 100)?;
    } else {
        let y = &mw.get_image_height() / 2;
        mw.extent_image(2200, 2200, 100, (1100 - y).try_into().unwrap())?;
    }

    let s = filename.split(".").next().unwrap();

    mw.write_image(format!("{}-square.jpg", s))?;
    Ok(())
}

fn make_square_from_blob(blob: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("yeet.jpg");
    let path = path.to_str().context("get image path failed")?;

    initialize();

    let mut binding = MagickWand::new();
    let mut mw = binding
        .read_image_blob(&blob)
        .unwrap()
        .set_image_gravity(graphicsmagick::types::GravityType::CenterGravity)
        .unwrap()
        .transform_image("", "2000x2000>")
        .expect("yeet");

    if mw.get_image_height() == 2000 {
        let x = &mw.get_image_width() / 2;
        mw.extent_image(2200, 2200, (1100 - x).try_into().unwrap(), 100)?;
    } else {
        let y = &mw.get_image_height() / 2;
        mw.extent_image(2200, 2200, 100, (1100 - y).try_into().unwrap())?;
    }

    Ok(mw.set_image_format("jpg")
        .unwrap()
        .write_image_blob()
        .unwrap()
        .to_vec())
}

async fn make_square_handler(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap().to_string().eq("file") {
            let data = field.bytes().await.unwrap().to_vec();

            let data2 = make_square_from_blob(data).unwrap();
            let headers = [
                (header::CONTENT_TYPE, "image/jpg"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"square.jpg\"",
                ),
            ];

            return (headers, data2).into_response();
        }
    }

    return ().into_response();
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/square", post(make_square_handler))
        .route("/", get(root))
        .layer(DefaultBodyLimit::max(4096));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3128").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> Html<String> {
    let html_text = std::fs::read_to_string("index.html").unwrap();
    Html(html_text)
}
