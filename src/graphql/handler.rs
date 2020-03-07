use std::sync::Arc;
// use crate::cli_args::Opt;
use actix_web::{error, web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use crate::graphql::schemas::root::{Context, Schema};
use juniper::http::GraphQLRequest;
use crate::database::{db_connection, Pool};

pub(crate) async fn graphql(
    pool: web::Data<Pool>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let db_pool = db_connection(&pool)?;
    println!("pspspsps");
    let ctx = Context::new(db_pool);

    let res = data.execute(&schema, &ctx);
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub(super) fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:3000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
