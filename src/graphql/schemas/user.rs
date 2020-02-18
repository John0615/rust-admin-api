use juniper;

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}
