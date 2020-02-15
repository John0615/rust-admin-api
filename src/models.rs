#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub salt: String,
    pub password_digest: String,
    pub phone: String,
    pub email: String,
    pub role: String,
    pub login_name: String,
    pub status: String,
    pub inserted_at: String,
    pub updated_at: String
}
