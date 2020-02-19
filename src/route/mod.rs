use actix_web::web;
use crate::app::user::{ aa, bb };


pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(web::resource("/").route(web::get().to(aa)))
            .service(web::resource("/again").route(web::get().to(bb)))
    );
}
