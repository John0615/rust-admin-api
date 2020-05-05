use crate::database::PooledConnection;
use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::schemas::root::Context;
use crate::jwt::manager::create_token;
use crate::jwt::model::Token;
use crate::modules::user::model::{SlimUser, Users};
use crate::modules::user::util::verify;
use diesel::prelude::*;

pub fn login(
    context: &Context,
    login_name: String,
    password_digest: String,
) -> ServiceResult<Token> {
    use crate::schema::users::dsl::{login_name as loginname, users};

    let conn: &PooledConnection = &context.db;
    let user = users
        .filter(loginname.eq(login_name))
        .first::<Users>(conn)
        .map_err(|_| ServiceError::Unauthorized)?;

    if verify(&user, &password_digest) {
        // 签发token
        // println!("psospsp");
        match create_token(
            &SlimUser {
                login_name: user.login_name,
                email: user.email,
                role: user.role,
            },
            24,
        ) {
            Ok(r) => Ok(Token { bearer: Some(r) }),
            Err(e) => Err(e),
        }
    // Ok(Logined{token: String::from("0w0w0w0w0")})
    } else {
        Err(ServiceError::Unauthorized)
    }
}

pub fn refresh_token(context: &Context) -> ServiceResult<Token> {
    println!("content>>>>>:{:?}", context.token.jwt);

    match context.token.jwt {
        None => Err(ServiceError::Unauthorized),
        Some(ref m) => {
            eprintln!("ksksk{:?}", m);
            match create_token(
                &SlimUser {
                    login_name: m.login_name.clone(),
                    email: m.email.clone(),
                    role: m.role.clone(),
                },
                24,
            ) {
                Ok(r) => Ok(Token { bearer: Some(r) }),
                Err(e) => Err(e),
            }
            // Ok(Token { bearer: Some(String::from("kkksk")) })
        },
    }
}
