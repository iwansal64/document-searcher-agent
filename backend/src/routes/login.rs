use std::{
        env::{self, VarError},
        str::from_utf8,
};

use actix_web::{HttpResponse, Responder, post, web};
use cookie::{Cookie, time::Duration};
use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::{AppState, utils::error_officer};

#[derive(Deserialize)]
struct LoginRequestData {
        password: String,
}

#[derive(Serialize)]
struct LoginResponseData {
        success: bool,
        message: String,
}

#[post("/login")]
async fn login(payload: web::Json<LoginRequestData>, state: web::Data<AppState>) -> impl Responder {
        // ? Check password
        let correct_password: Result<String, VarError> = env::var("AUTHENTICATION_PASSWORD");
        let correct_password: String = match correct_password {
                Ok(password) => password,
                Err(_) => {
                        error_officer::error_env("AUTHENTICATION_PASSWORD");
                        return HttpResponse::InternalServerError().json(LoginResponseData {
                                success: false,
                                message: String::from("There's an error with server"),
                        });
                }
        };

        if payload.password != correct_password {
                return HttpResponse::Unauthorized().json(LoginResponseData {
                        success: false,
                        message: String::from("Unauthorized"),
                });
        }

        // ? Create session token
        let token_length_res: Result<String, VarError> = env::var("SESSION_TOKEN_LENGTH");
        let token_length_str = match token_length_res {
                Ok(length) => length,
                Err(_) => {
                        error_officer::error_env("SESSION_TOKEN_LENGTH");
                        return HttpResponse::InternalServerError().json(LoginResponseData {
                                success: false,
                                message: String::from("There's an error with server"),
                        });
                }
        };

        let token_length = token_length_str.parse();
        let token_length = match token_length {
                Ok(token_length) => token_length,
                Err(_) => {
                        error_officer::error_parse("token_length");
                        return HttpResponse::InternalServerError().json(LoginResponseData {
                                success: false,
                                message: String::from("There's an error with server"),
                        });
                }
        };

        let mut rng = rand::rng();
        let mut session_key: String = String::new();

        for _ in 0..token_length {
                // Get random decimal
                let mut choosen_index: [u8; 1] = [rng.random_range(65..=122 - 6)];
                // Skip symbols
                if choosen_index[0] > 90 {
                        choosen_index[0] += 6;
                }
                // Convert to character according to ascii code table
                session_key += from_utf8(&choosen_index).unwrap();
        }

        // ? Store session token to DB
        let insert_result = sqlx::query!(
                r#"INSERT INTO "accessToken"(token) VALUES ($1);"#,
                session_key
        )
        .execute(&state.db)
        .await;

        match insert_result {
                Ok(_) => (),
                Err(err) => {
                        error_officer::error_db(&err.to_string());
                }
        }

        // ? Create cookie
        let cookie = Cookie::build("access_token", session_key)
                .max_age(Duration::days(7))
                .same_site(cookie::SameSite::Lax)
                .finish();

        // ? Return OK response
        HttpResponse::Ok().cookie(cookie).json(LoginResponseData {
                success: true,
                message: String::from("Successfully login!"),
        })
}
