use crate::database::Pool;
use crate::user::model::Users;
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
        Ok(vec![Users {
            id: 1,
            salt: String::from("11111"),
            password_digest: String::from("22222"),
            phone: String::from("33333"),
            email: String::from("44444"),
            role: String::from("55555"),
            login_name: String::from("66666"),
            status: String::from("77777"),
            inserted_at: String::from("88888"),
            updated_at: String::from("99999"),
        }])
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
