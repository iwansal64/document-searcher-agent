use std::env;

use actix_web::{
        body::MessageBody,
        dev::{ServiceRequest, ServiceResponse},
        middleware::Next,
        web,
};

use crate::{
        AppState,
        db::model::AccessToken,
        utils::error_officer::{self, generate_server_error_response},
};

pub async fn routes_lock(
        state: web::Data<AppState>,
        req: ServiceRequest,
        next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
        // ? Get unprotected endpoints
        let unprotected_endpoints = env::var("UNPROTECTED_ENDPOINTS");
        let unprotected_endpoints: String = match unprotected_endpoints {
                Ok(data) => data,
                Err(_) => {
                        error_officer::error_env("UNPROTECTED_ENDPOINTS");
                        return Ok(ServiceResponse::new(
                                req.into_parts().0,
                                generate_server_error_response(),
                        ));
                }
        };
        let unprotected_endpoints: Vec<&str> = unprotected_endpoints.split(",").collect();

        // ? Check if the current URL not in protected endpoints
        if !unprotected_endpoints.contains(&req.uri().to_string().as_str()) {
                let access_token_cookie = req.cookie("access_token").map(|data| data.to_string());
                let access_token_cookie = match access_token_cookie {
                        Some(data) => data,
                        None => {
                                return Ok(ServiceResponse::new(
                                        req.into_parts().0,
                                        error_officer::generate_unauthorized_response(),
                                ));
                        }
                };

                let access_token_db: Result<Option<AccessToken>, sqlx::Error> = sqlx::query_as!(
                        AccessToken,
                        r#"SELECT * FROM "accessToken" WHERE token=$1"#,
                        access_token_cookie
                )
                .fetch_optional(&state.db)
                .await;

                match access_token_db {
                        Ok(token) => {
                                match token {
                                        Some(_) => (),
                                        None => return Ok(ServiceResponse::new(
                                                req.into_parts().0,
                                                error_officer::generate_unauthorized_response(),
                                        )),
                                }
                        }
                        Err(err) => {
                                error_officer::error_db(&err.to_string());
                                return Ok(ServiceResponse::new(
                                        req.into_parts().0,
                                        error_officer::generate_server_error_response(),
                                ));
                        }
                }
        }

        // ? Pass the request
        let res = next.call(req).await?;

        Ok(res.map_into_boxed_body())
}
