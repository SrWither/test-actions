use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::Surreal;
use std::sync::Arc;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

static SURREALDB: Lazy<Arc<Mutex<Option<Surreal<Client>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

pub async fn initialize_db() -> surrealdb::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>("127.0.0.1:7432").await?;
    db.use_ns("blog").use_db("blog").await?;
    Ok(db)
}

pub async fn get_db() -> surrealdb::Result<Surreal<Client>> {
    let surreal_mutex = SURREALDB.clone();

    let mut surreal_lock = surreal_mutex.lock().await;

    if surreal_lock.is_none() {
        let db = initialize_db().await?;
        *surreal_lock = Some(db);
    }

    Ok(surreal_lock.as_ref().unwrap().clone())
}
