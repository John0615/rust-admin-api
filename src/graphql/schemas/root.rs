use crate::app::user::model::Users;
use crate::app::user::service as user;
use crate::database::Pool;
use juniper::Context as JuniperContext;
use juniper::FieldResult;

// use crate::user::service as user;
use juniper;
use juniper::RootNode;

pub struct Context {
    pub dbpool: Pool,
}

impl JuniperContext for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(context: &Context) -> FieldResult<Vec<Users>> {
        user::list::find_all_users(&context)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
