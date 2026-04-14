use super::{DbError};

pub async fn load_async<F>(f: F) -> Result<Vec<Vec<String>>, String>
where
    F: FnOnce() -> Result<Vec<Vec<String>>, DbError> + Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}