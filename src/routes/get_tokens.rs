use actix_web::{get, web, HttpResponse, Responder};
use crate::{token_listings, EnvVar};
use crate::utils::response::{SuccessResponse, ErrorResponse};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct PathParams {
    dex: String,
}

async fn fetch_tokens_from_dex<'a, F, T>(fetch_fn: F, env_var: &'a EnvVar) -> HttpResponse
where
    F: Fn(&'a EnvVar) -> T,
    T: std::future::Future<Output = Result<serde_json::Value, Box<dyn std::error::Error>>> + 'a,
{
    match fetch_fn(env_var).await {
        Ok(body) => {
            let response = SuccessResponse {
                data: body,
                message: "Success".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: "InternalServerError".to_string(),
                message: format!("Failed to fetch tokens: {}", e),
            };
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

#[get("/latest/token/{dex}")]
pub async fn fetch_tokens(path_param: web::Path<PathParams>, app_state: web::Data<EnvVar>) -> impl Responder {
    let dex_param = path_param.into_inner();
    let dex = dex_param.dex.to_lowercase();
    // let env_var = app_state.birdeye_api_key.clone();
    let env_var = app_state.get_ref();

    match dex.as_str() {
        "dexscreener" => fetch_tokens_from_dex(|env| token_listings::dexscreener::fetch_tokens(env), env_var).await,
        "birdeye" => fetch_tokens_from_dex(|env| token_listings::birdeye::get_trending_tokens(env), env_var).await,
        _ => {
            let error_response = ErrorResponse {
                error: "BadRequest".to_string(),
                message: "Dex not supported".to_string(),
            };
            HttpResponse::BadRequest().json(error_response)
        }
    }
}
