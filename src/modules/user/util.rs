use super::model::Users;
use md5::compute;


pub fn verify(user: &Users, password: &str) -> bool {
    let Users { password_digest,salt, .. } = user;
    let compute_password = format!("{:x}", compute(format!("{}{}", password, salt)));
    &compute_password == password_digest
}

