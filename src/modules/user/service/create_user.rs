use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::schemas::root::Context;
use crate::modules::user::model::{Users, InsertableUser};
use diesel::prelude::*;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

pub(crate) fn create_user(context: &Context, limit: usize) -> ServiceResult<Vec<Users>> {
    let day = NaiveDate::from_ymd(2015, 6, 3);
    let time = NaiveTime::from_hms_milli(12, 34, 56, 789);
    let datetime = NaiveDateTime::new(day, time);
    let data = InsertableUser {
        salt: String::from("11111"),
        password_digest: String::from("22222"),
        phone: String::from("33333"),
        email: String::from("44444"),
        role: String::from("55555"),
        login_name: String::from("66666"),
        status: String::from("77777"),
        inserted_at: datetime,
    };

    use crate::schema::users::dsl::*;
    let conn: &PooledConnection = &context.db;


    let user: InsertableUser = data.into();

    let inserted_user: Users = diesel::insert_into(users).values(&user).get_result(conn)?;
    println!("{:?}", inserted_user);

    Ok(vec![Users {
        id: 1,
        salt: String::from("11111"),
        password_digest: String::from("22222"),
        phone: String::from("33333"),
        email: String::from("44444"),
        role: String::from("55555"),
        login_name: String::from("66666"),
        status: String::from("77777"),
        inserted_at: datetime,
        updated_at: Some(datetime),
    }])
}
