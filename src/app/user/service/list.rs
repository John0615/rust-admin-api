// use crate::database::PooledConnection;
// use crate::errors::ServiceResult;
// use crate::graphql::schemas::root::Context;
// use crate::user::model::Users;
// use diesel::prelude::*;

// pub(crate) fn find_all_users(context: &Context, limit: usize) -> ServiceResult<Vec<Users>> {
//     use crate::schema::users::dsl::*;
//     let conn: &PooledConnection = &context.db;

//     Ok(users.limit(limit as i64).load::<Users>(conn)?)
// }
