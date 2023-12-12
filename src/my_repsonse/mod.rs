use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, http::StatusCode};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiPageResponse<T>{
    pub page: i32,
    pub pages:i32,
    pub total: i32,
    pub records: Option<T>,

}
impl<T> ApiPageResponse<T>{
    pub fn new(records:Option<T>,page:i32,pages:i32,total:i32)-> Self{
        Self{
            records,
            page,
            pages,
            total
        }
    }
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

pub fn json_page_response<T:Serialize>(status: StatusCode,message: &str, data: Option<T>,page:i32,pages:i32,total:i32) -> HttpResponse {
    let api_response = ApiResponse::new(status, message, Some(ApiPageResponse::new(data, page, pages, total)));
    HttpResponse::build(status).json(api_response)
}