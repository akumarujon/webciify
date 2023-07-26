use std::{fs::File, io::{self, Write}};
use std::fs::OpenOptions;


use axum::{routing::get, Router, Json, extract::Query};
use serde_json::{Value,json};

use tapciify::{image_to_ascii, DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};

mod structs;

use structs::{AsciiResponse, QueryParams};

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(hello_world));
    let host = "http://localhost:3000";

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn download_image(image_link: String) -> String {
    let image_path = "./assets/image.png";
    let mut file = OpenOptions::new().read(true).write(true).open(image_path).expect("Could not load");
    let img_bytes = reqwest::get(image_link).await.expect("Could not load")
    .bytes().await.expect("Could not");

    file.write_all(&img_bytes).expect("COuld nit");

    image_path.to_string()
}

async fn hello_world(query: Query<QueryParams>) -> Json<AsciiResponse> {
    let image = image::open(download_image(query.image.clone()).await);

    let ascii_img = image_to_ascii(
        image.unwrap(),
        query.width,
        DEFAULT_ASCII_STRING, 
        false, 
        DEFAULT_FONT_RATIO);
    println!("{}",&query.image);
    

    Json(AsciiResponse { message: (ascii_img.result) })
}