use chrono::*;
use crate::schema::*;

#[derive(Debug, Queryable, juniper::GraphQLObject)]
pub struct Users {
    pub id: i32,
    pub salt: String,
    pub password_digest: String,
    pub phone: String,
    pub email: String,
    pub role: String,
    pub login_name: String,
    pub status: String,
    pub inserted_at: NaiveDateTime,
    pub updated_at: Option<naive::NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub salt: String,
    pub password_digest: String,
    pub phone: String,
    pub email: String,
    pub role: String,
    pub login_name: String,
    pub status: String,
    pub inserted_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct SlimUser {
    pub login_name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Queryable, juniper::GraphQLObject)]
pub struct Logined {
    pub token: String,
}
