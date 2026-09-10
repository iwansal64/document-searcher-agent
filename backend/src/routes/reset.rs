use std::env::{self, VarError};

use actix_web::{HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{AppState, routes::ResponseData, utils::error_officer};

pub async fn reset(auth: BearerAuth, state: web::Data<AppState>) -> HttpResponse {
        // ? Check authorization
        let correct_reset_password: Result<String, VarError> = env::var("RESET_PASSWORD");
        let correct_reset_password: String = match correct_reset_password {
                Ok(password) => password,
                Err(_) => {
                        return HttpResponse::InternalServerError().json(ResponseData {
                                success: false,
                                message: String::from("There's server error"),
                        });
                }
        };

        let user_reset_password = auth.token();
        if correct_reset_password != user_reset_password {
                return HttpResponse::Unauthorized().json(ResponseData {
                        success: false,
                        message: String::from("Unauthorized Access"),
                });
        }

        // ? Remove all of the access token
        let remove_access_token_result = sqlx::query!(r#"DELETE FROM "accessToken""#)
                .execute(&state.db)
                .await;

        match remove_access_token_result {
                Ok(_) => (),
                Err(err) => {
                        error_officer::error_db(&err.to_string());
                        return HttpResponse::InternalServerError().json(ResponseData {
                                success: false,
                                message: String::from(
                                        "An error just occured when removing access token",
                                ),
                        });
                }
        }

        // Return OK http response
        HttpResponse::Ok().finish()
}
