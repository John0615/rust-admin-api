use crate::errors::ServiceError;
use crate::jwt::model::Claims;
use crate::modules::user::model::SlimUser;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation, EncodingKey, DecodingKey};

pub fn create_token(
    user: &SlimUser,
    auth_duration_in_hour: u16,
) -> Result<String, ServiceError> {
    let claims: Claims = Claims::new(user, auth_duration_in_hour);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<Claims, ServiceError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::RS256),
    )
    .map(|data| data.claims)
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}
