
use actix_web::{Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;

pub(super) async fn graphql() -> Result<HttpResponse, Error> {


    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("aa"))
}

pub(super) fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:3000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
