use tapciify::{image_to_ascii, DEFAULT_ASCII_STRING, DEFAULT_FONT_RATIO};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    let image = image::open("./assets/original.png").unwrap();
    let ascii_image = image_to_ascii(image, 64, DEFAULT_ASCII_STRING, false, DEFAULT_FONT_RATIO);
    
    let result = ascii_image.result;
    
    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}