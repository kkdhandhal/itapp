
use actix_web::{http::{StatusCode},HttpResponse, ResponseError, body::BoxBody};
use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct MyError{
    pub id:u32,
    pub msg:String,
}

impl ResponseError for MyError {
    
    fn status_code(&self) -> StatusCode{
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        let res = HttpResponse::new(self.status_code());
        res.set_body(BoxBody::new(body))
    }
}

impl std::fmt::Display for MyError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}