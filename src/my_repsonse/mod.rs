use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, http::StatusCode};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(status: StatusCode, message: &str, data: Option<T>) -> Self {
        Self {
            status: status.to_string(),
            message: message.to_string(),
            data,
        }
    }
}

pub fn json_response<T: Serialize>(status: StatusCode, message: &str, data: Option<T>) -> HttpResponse {
    let api_response = ApiResponse::new(status, message, data);
    HttpResponse::build(status).json(api_response)
}
