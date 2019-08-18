use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    message: String,
}
pub fn new_transaction(req: web::Json<Info>) -> HttpResponse {
    HttpResponse::Ok().json(req.0)
}

pub fn mine(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Mining a new block")
}

pub fn chain(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Here you go")
}
