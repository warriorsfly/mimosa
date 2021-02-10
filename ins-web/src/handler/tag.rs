use crate::{database::DatabasePool, db, errors::ServError, helpers::respond_json};
use actix_web::web::{block, Data, Json};

pub async fn get_tags(pool: Data<DatabasePool>) -> Result<Json<Vec<String>>, ServError> {
    let tags = block(move || db::get_tags(&pool)).await?;

    respond_json(tags)
}