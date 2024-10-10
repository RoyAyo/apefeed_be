use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod routes;
mod token_listings;
mod utils;

use routes::{get_tokens, get_top_traders};

struct EnvVar {
    birdeye_api_key: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Trader")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let env_var = web::Data::new( EnvVar {
        birdeye_api_key: std::env::var("BIRDEYE_API_KEY").expect("BIRDEYE_API_KEY must be set"),
    } );

    HttpServer::new(move || {
        App::new()
            .app_data(env_var.clone())
            .service(index)
            .service(get_tokens::fetch_tokens)
            .service(get_top_traders::get_top_traders)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
