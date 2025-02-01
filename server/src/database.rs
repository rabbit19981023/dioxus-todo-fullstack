use sqlx::PgPool;
use tokio::sync::OnceCell;

static DB: OnceCell<PgPool> = OnceCell::const_new();

async fn init_db() -> PgPool {
    PgPool::connect("postgresql://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap()
}

pub async fn get_db() -> &'static PgPool {
    DB.get_or_init(init_db).await
}
