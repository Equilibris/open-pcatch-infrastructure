mod api_types;
mod auth;
mod db_types;
mod global_pool;
mod service;

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;

pub fn router() -> Router {
    Router::new().route(
        "/service/object-types",
        get(service::object_types::get).post(service::object_types::post),
    )
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    sqlx::migrate!();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env!("DATABASE_URL"))
        .await?;

    global_pool::populate(pool).unwrap();

    let app = router();

    println!("{:#?}", app);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
