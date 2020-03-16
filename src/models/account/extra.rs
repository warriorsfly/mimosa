use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono;
use diesel::prelude::*;
use futures::future::{ready, Ready};
use serde::Serialize;
// use uuid::Uuid;

// use crate::{
//     config::db::Connection,
//     constants,
//     entity::{
//         // login_history::LoginHistory,
//         user::token::UserToken,
//     },
//     schema::{
//         user_extra::{self, dsl::*},
//     }
// };

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserExtra {
    pub uid: String,
    pub vendor: String,
    pub client_name: String,
    pub client_version: String,
    pub os_name: String,
    pub os_version: String,
    pub device_id: String,
    pub device_name: String,
    pub idfa: String,
    pub idfv: String,
    pub market: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub extend1: String,
    pub extend2: String,
    pub extend3: String,
}

impl Responder for UserExtra {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}