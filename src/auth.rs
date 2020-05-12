use crate::config::CONFIG;
use crate::errors::ServiceError;
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use derive_more::Display;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialEq)]
pub enum Claim {
    /// 手机登录
    #[display(fmt = "")]
    Phone { uid: Uuid, phone: String },
    /// 邮箱登录
    #[display(fmt = "")]
    Email { uid: Uuid, email: String },
    /// 邮箱登录
    #[display(fmt = "")]
    Name { uid: Uuid, name: String },
    /// QQ登录
    #[display(fmt = "")]
    QQ { uid: Uuid, qq: String },
    /// 邮箱登录
    #[display(fmt = "")]
    Wechat { uid: Uuid, openid: String },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AccountClaim {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub username: String,
}

impl AccountClaim {
    pub fn new(id: i32, username: String) -> Self {
        Self {
            id,
            username,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub fn create_jwt(private_claim: AccountClaim) -> Result<String, ServiceError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &private_claim, &encoding_key)
        .map_err(|e| ServiceError::EncodeTokenError(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<AccountClaim, ServiceError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<AccountClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ServiceError::DecodeTokenError(e.to_string()))
}

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// salt is environment-condigured.
pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    static PHONE: &str = "18326069658";
    static ID: i32 = 1;

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let hashed = hash(password);
        assert_ne!(password, hashed);
    }

    #[test]
    fn it_matches_2_hashed_passwords() {
        let password = "password";
        let hashed = hash(password);
        let hashed_again = hash(password);
        println!("{}", hashed);
        println!("{}", hashed_again);
        assert_eq!(hashed, hashed_again);
    }

    #[test]
    fn it_creates_a_jwt() {
        let private_claim = AccountClaim::new(ID, PHONE.into());
        let jwt = create_jwt(private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        let private_claim = AccountClaim::new(ID, PHONE.into());
        let jwt = create_jwt(private_claim.clone()).unwrap();
        let decoded = decode_jwt(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }
}
