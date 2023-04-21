use crate::{api_types, db_types, global_pool};

pub async fn get() -> Result<axum::Json<Vec<api_types::ObjectType>>, &'static str> {
    let pool = global_pool::get();

    let data = sqlx::query_as!(crate::db_types::ObjectType, "SELECT * FROM ObjectType")
        .fetch_all(pool)
        .await
        .map_err(|_| "Failde to fetch")?;

    Ok(axum::Json(data.into_iter().map(Into::into).collect()))
}

pub async fn post(
    axum::extract::Json(val): axum::extract::Json<api_types::ObjectType>,
) -> Result<(), &'static str> {
    let pool = global_pool::get();

    db_types::ObjectType::from(val)
        .insert(&pool)
        .await
        .map_err(|_| "lol")?;

    Ok(())
}
