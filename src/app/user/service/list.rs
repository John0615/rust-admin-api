// use crate::database::PooledConnection;
// use crate::errors::ServiceResult;
use crate::graphql::schemas::root::Context;
use crate::app::user::model::Users;
use juniper::FieldResult;

// use diesel::prelude::*;

pub(crate) fn find_all_users(context: &Context) -> FieldResult<Vec<Users>> {
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
