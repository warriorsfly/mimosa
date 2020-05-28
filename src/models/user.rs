use crate::{
    auth::{create_jwt, JwtAccount},
    errors::ServiceError,
    schema::users,
};
use serde::Serialize;
use validator::Validate;

type Url = String;

#[derive(Insertable, Serialize, Validate)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<Url>,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Serialize)]
pub struct AccountAuth<'a> {
    username: &'a str,
    email: &'a str,
    bio: Option<&'a str>,
    image: Option<&'a str>,
    token: String,
}

#[derive(Serialize)]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl User {
    pub fn to_user_auth(&self) -> Result<AccountAuth, ServiceError> {
        let token = create_jwt(JwtAccount::new(self.id, self.username.clone()))?;

        Ok(AccountAuth {
            username: &self.username,
            email: &self.email,
            bio: self.bio.as_ref().map(String::as_str),
            image: self.image.as_ref().map(String::as_str),
            token,
        })
    }

    pub fn to_profile(self, following: bool) -> Profile {
        Profile {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following,
        }
    }
}
