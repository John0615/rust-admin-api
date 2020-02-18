use crate::database::{Pool};
use juniper;
use juniper::{FieldResult, RootNode};
use super::user::{User};

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {

}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
