use actix_web::{ web, HttpRequest};
use async_graphql_actix_web::{GQLRequest};
use async_graphql::http::{GQLResponse};

use async_graphql::{Context, FieldResult, Schema, EmptyMutation};

use futures::{stream, Stream};

pub type MySchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub struct MyToken(String);

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
}


pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> FieldResult<impl Stream<Item = i32>> {
        if ctx.data::<MyToken>().0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(stream::once(async move { 10 }))
    }
}

pub async fn index(
  schema: web::Data<MySchema>,
  req: HttpRequest,
  gql_request: GQLRequest,
) -> web::Json<GQLResponse> {
  let token = req
      .headers()
      .get("Token")
      .and_then(|value| value.to_str().map(|s| MyToken(s.to_string())).ok());
  let mut query = gql_request.into_inner();
  if let Some(token) = token {
      query = query.data(token);
  }
  web::Json(GQLResponse(query.execute(&schema).await))
}
