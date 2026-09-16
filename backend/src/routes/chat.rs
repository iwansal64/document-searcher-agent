use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

use crate::{AppState, routes::ResponseData, utils::error_officer};

#[derive(Deserialize)]
struct ChatRequestBody {
        message: String,
}

#[post("/chat")]
pub async fn chat(body: web::Json<ChatRequestBody>, state: web::Data<AppState>) -> impl Responder {
        // ? Send response
        let response_result = state.into_inner().ai_model.ask(&body.message).await;

        let ai_response = match response_result {
                Ok(res) => res,
                Err(err) => {
                        error_officer::error_db(err.as_str());
                        return error_officer::generate_server_error_response();
                }
        };

        HttpResponse::Ok().json(ResponseData {
                success: true,
                message: ai_response,
        })
}
