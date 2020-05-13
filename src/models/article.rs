use crate::config::DATE_FORMAT;
use crate::models::user::User;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: i32,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorites_count: i32,
}

impl Article {
    pub fn attach(self, author: User, favorited: bool) -> ArticleJson {
        ArticleJson {
            id: self.id,
            title: self.title,
            description: self.description,
            body: self.body,
            author,
            tags: self.tags,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
            favorites_count: self.favorites_count,
            favorited,
        }
    }
}

#[derive(Serialize)]
pub struct ArticleJson {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: User,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub favorites_count: i32,
    pub favorited: bool,
}