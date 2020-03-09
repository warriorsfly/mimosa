use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono;
use diesel::prelude::*;
use serde::Serialize;

// use crate::{
//     config::db::Connection,
//     constants,
//     entity::user::token::UserToken,
//     schema::user_auth::{self, dsl::*},
// };

//表实体
#[derive(Debug, Serialize, Deserialize, Queryable)]
// #[table_name = "user_auth"]
pub struct UserAuth {
    pub id: String,
    pub uid: String,
    pub identity_type: i32,
    pub identifier: String,
    pub certificate: String,
    pub login_session: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthAccount {
    /// user id, the user-base id
    pub uid: String,
    pub phone: String,
}

impl UserAuth {}
