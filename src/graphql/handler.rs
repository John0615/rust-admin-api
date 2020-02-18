use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use crate::graphql::schemas::root::{Context, Schema};
use juniper::http::GraphQLRequest;
use crate::database::Pool;

pub(super) async fn graphql(
    pool: web::Data<Pool>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {

    let ctx = Context {
        dbpool: pool.get_ref().to_owned(),
    };
    let res = web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

pub(super) fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:3000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
