use serde::Serialize;

pub mod login;
pub mod reset;

#[derive(Serialize)]
struct ResponseData {
        success: bool,
        message: String,
}
