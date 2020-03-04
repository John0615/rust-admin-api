use actix_web::web;
pub mod schemas;
pub mod handler;

pub(super) fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to(handler::graphql)))
        .service(web::resource("/graphiql").route(web::get().to(handler::graphiql)));
}
