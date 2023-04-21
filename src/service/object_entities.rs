use crate::{api_types, db_types, global_pool};

pub async fn post(axum::Json(val): axum::Json<api_types::ObjectEntity>) -> Result<(), ()> {
    let pool = global_pool::get();

    let (obj, ty, attrs) = val.into_db();

    obj.insert(&pool).await.unwrap();

    // Can cause consitency errors but probs not
    if let None = sqlx::query!("SELECT ky FROM ObjectType WHERE ky = $1", ty.ky)
        .fetch_optional(pool)
        .await
        .unwrap()
    {
        ty.insert(&pool).await.unwrap();
    }

    let mut a = Vec::new();
    let mut b = Vec::new();

    for (val, vax) in attrs {
        a.push(val);
        b.push(vax);
    }

    Ok(())
}
