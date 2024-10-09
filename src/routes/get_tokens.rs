use actix_web::{get, web, HttpResponse, Responder};
use crate::token_listings;
use crate::utils::response::{SuccessResponse, ErrorResponse};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct PathParams {
    dex: String,
}

#[get("/latest/token/{dex}")]
pub async fn fetch_tokens(path_param: web::Path<PathParams>) -> impl Responder {
    let dex_param = path_param.into_inner();
    let dex = dex_param.dex.to_lowercase();
    if dex == "dexscreener" {
        match token_listings::dexscreener::fetch_tokens().await {
            Ok(body) => {
                let response = SuccessResponse {
                    data:  body,
                    message: "Success".to_string(),
                };

                return HttpResponse::Ok().json(response);
            },
            Err(e) => {
                let error_response = ErrorResponse {
                    error: "InternalServerError".to_string(),
                    message: format!("Failed to fetch tokens from dexscreener: {}", e),
                };
                return HttpResponse::InternalServerError().json(error_response);
            },
        }
    } 
    else if dex == "birdeye" {
        match token_listings::birdeye::get_trending_tokens().await {
            Ok(body) => {
                let response = SuccessResponse {
                    data:  body,
                    message: "Success".to_string(),
                };

                return HttpResponse::Ok().json(response);
            },
            Err(e) => {
                let error_response = ErrorResponse {
                    error: "InternalServerError".to_string(),
                    message: format!("Failed to fetch tokens from birdeye: {}", e),
                };
                return HttpResponse::InternalServerError().json(error_response);
            },
        }
    }
    else {
        let error_response = ErrorResponse {
            error: "BadRequest".to_string(),
            message: "Dex not supported".to_string(),
        };

        return HttpResponse::BadRequest().json(error_response);
    }
}