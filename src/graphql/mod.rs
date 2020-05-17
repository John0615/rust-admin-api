use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source};

pub mod schemas;
pub mod handler;
pub mod index;

// pub(super) fn route(cfg: &mut web::ServiceConfig) {
//     cfg.service(web::resource("/graphql").route(web::post().to(handler::graphql)))
//         .service(web::resource("/graphiql").route(web::get().to(handler::graphiql)));
// }

pub(super) async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", Some("/graphql")))
}
