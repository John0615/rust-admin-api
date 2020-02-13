use actix_web::{HttpResponse, Responder};

pub async fn aa() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

pub async fn bb() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}
