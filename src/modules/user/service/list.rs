use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::modules::user::model::{Users};
use crate::graphql::schemas::root::Context;
use diesel::prelude::*;

pub(crate) fn find_all_users(context: &Context, limit: usize) -> ServiceResult<Vec<Users>> {
    use crate::schema::users::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(users.limit(limit as i64).load::<Users>(conn)?)
    // Ok(vec![Users {
    //     id: 1,
    //     salt: String::from("11111"),
    //     password_digest: String::from("22222"),
    //     phone: String::from("33333"),
    //     email: String::from("44444"),
    //     role: String::from("55555"),
    //     login_name: String::from("66666"),
    //     status: String::from("77777"),
    //     inserted_at: String::from("88888"),
    //     updated_at: String::from("99999"),
    // }])
}
