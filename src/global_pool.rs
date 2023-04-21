use sqlx::PgPool;
use tokio::sync::OnceCell;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub fn populate(pool: PgPool) -> Result<(), &'static str> {
    POOL.set(pool).map_err(|_| "Failed to populate once")
}
pub fn get() -> &'static sqlx::Pool<sqlx::Postgres> {
    POOL.get().unwrap()
}
