use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod token_listings;
mod routes;
mod utils;

use routes::get_tokens;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Trader")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_tokens::fetch_tokens)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
