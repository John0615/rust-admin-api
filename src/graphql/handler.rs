
use actix_web::{Error, web, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use crate::cli_args::Opt;

pub(super) async fn graphql() -> Result<HttpResponse, Error> {


    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("aa"))
}

pub(super) fn graphiql(opt: web::Data<Opt>) -> HttpResponse {
    let html = graphiql_source(&format!("http://127.0.0.1:{}/graphql", opt.port));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
