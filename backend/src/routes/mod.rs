use serde::Serialize;

pub mod chat;
pub mod login;
pub mod middleware;
pub mod reset;

#[derive(Serialize)]
pub struct ResponseData {
        pub success: bool,
        pub message: String,
}
