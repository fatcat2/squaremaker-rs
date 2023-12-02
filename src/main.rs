use graphicsmagick::{initialize, types::FilterTypes, types::GravityType, wand::MagickWand};
use std::path::PathBuf;
use anyhow::Context;

use axum::{
    routing::{get, post},
    http::StatusCode,
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
        .transform_image("", "2000x2000>").expect("yeet");

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

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3128").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
