use crate::modules::user::model::SlimUser;
use anyhow::Result;
use chrono::{Duration, Local};
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone)]
pub struct DecodedToken {
    pub jwt: Option<Claims>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // login_name
    pub login_name: String,
    // issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user email
    pub email: String,
    // user role
    pub role: String,
}

// struct to get converted to token and back
impl Claims {
    pub(crate) fn new(slim_user: &SlimUser, auth_duration_in_hour: u16) -> Self {
        let SlimUser {
            login_name,
            email,
            role,
            ..
        } = slim_user;

        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Claims {
            login_name: login_name.clone(),
            email: email.clone(),
            role: role.clone(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Token {
    pub bearer: Option<String>,
}

impl TryFrom<Claims> for SlimUser {
    type Error = anyhow::Error;

    fn try_from(claims: Claims) -> Result<Self> {
        let Claims {
            login_name,
            email,
            role,
            ..
        } = claims;

        Ok(SlimUser {
            login_name,
            email,
            role,
        })
    }
}
