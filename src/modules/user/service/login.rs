use crate::database::PooledConnection;
use crate::errors::{ServiceError, ServiceResult};
use crate::modules::user::model::{Logined, Users};
use crate::graphql::schemas::root::Context;
use crate::modules::user::util::verify;
use diesel::prelude::*;

pub fn login(
    context: &Context,
    login_name: String,
    password_digest: String,
) -> ServiceResult<Logined> {
    use crate::schema::users::dsl::{login_name as loginname, users};

    let conn: &PooledConnection = &context.db;
    let user = users
        .filter(loginname.eq(login_name))
        .first::<Users>(conn)
        .map_err(|_| ServiceError::Unauthorized)?;

    if verify(&user, &password_digest) {
        Ok(Logined{token: String::from("0w0w0w0w0")})
    } else {
        Err(ServiceError::Unauthorized)
    }
}
