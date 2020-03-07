use crate::jwt::model::{DecodedToken, Token};
use crate::modules::user::model::Users;
use crate::modules::user::service as user;
// use crate::cli_args::Opt;
use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use juniper::Context as JuniperContext;
use std::sync::Arc;

// use crate::user::service as user;
use juniper;
use juniper::RootNode;

pub struct Context {
    pub db: Arc<PooledConnection>,
    pub token: DecodedToken,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(pool: PooledConnection, token: DecodedToken,) -> Self {
        Self { db: Arc::new(pool), token }
    }
}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(context: &Context) -> ServiceResult<Vec<Users>> {
        user::list::find_all_users(&context, 10)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    pub fn login(
        context: &Context,
        login_name: String,
        password_digest: String,
    ) -> ServiceResult<Token> {
        user::login::login(&context, login_name, password_digest)
    }

    pub fn refresh_token(
        context: &Context,
        token: String,
    ) -> ServiceResult<Token> {
        user::login::refresh_token(&context)
    }
}
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
