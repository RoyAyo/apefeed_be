use actix_web::{get, HttpResponse};

use crate::{token_listings, utils::response::{ErrorResponse, SuccessResponse}};

#[get("/top_traders")]
async fn get_top_traders() -> HttpResponse {
    match token_listings::birdeye::get_top_traders().await {
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
                message: format!("Failed to fetch top traders: {}", e),
            };
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}