use std::path::PathBuf;
use axum::{routing::get, Router, Json, extract::Query};
use tapciify::{image_to_ascii, DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};
mod structs;
use structs::{AsciiResponse, QueryParams};


#[shuttle_runtime::main]
async fn axum(#[shuttle_static_folder::StaticFolder(folder = "assets")] public_folder: PathBuf) -> shuttle_axum::ShuttleAxum {
    println!("{:?}", &public_folder);

    let router = Router::new()
        .route("/", get(send_ascii));

    Ok(router.into())
}

async fn send_ascii(query: Query<QueryParams>) -> Json<AsciiResponse> {
    let img_bytes = reqwest::get(&query.image).await.unwrap().bytes().await.unwrap();

    let image = image::load_from_memory(&img_bytes).unwrap();
    
    let ascii_img = image_to_ascii(
        image,
        query.width,
        DEFAULT_ASCII_STRING, 
        false, 
        DEFAULT_FONT_RATIO);
    println!("{}", &query.image);
    

    Json(AsciiResponse { message: (ascii_img.result) })
}