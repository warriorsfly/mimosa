use crate::{
    auth::PrivateClaim,
    cache::{Cache},
    database::DatabasePoolType,
    db,
    errors::ServiceError,
    helpers::respond_json,
    models::ArticleJson,
    validate::validate,
};
use actix_web::web::{block, Data, Json};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewArticle {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 1))]
    description: String,
    #[validate(length(min = 1))]
    body: String,
    tags: Vec<String>,
}
pub struct ArticleRequest {}
pub async fn post_article(
    pool: Data<DatabasePoolType>,
    redis: Cache,
    claim: PrivateClaim,
    params: Json<NewArticle>,
) -> Result<Json<ArticleJson>, ServiceError> {
    validate(&params)?;
    let new_article = block(move || {
        db::create_article(
            &pool,
            &claim.id,
            &params.title,
            &params.description,
            &params.body,
            &params.tags,
        )
    })
    .await?;
    respond_json(new_article)
}
