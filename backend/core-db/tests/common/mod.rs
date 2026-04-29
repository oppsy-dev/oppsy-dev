use core_db::CoreDb;

#[allow(clippy::unwrap_used)]
pub async fn init_db() -> CoreDb {
    CoreDb::new("sqlite://oppsy.db").await.unwrap()
}
