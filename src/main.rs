use axum::{routing::get, Router, Json, extract::Query};
use tapciify::{image_to_ascii, DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};
mod structs;
use structs::{AsciiResponse, QueryParams};


#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(send_ascii));


    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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