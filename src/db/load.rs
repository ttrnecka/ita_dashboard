use super::queries;
use super::DbError;
use crate::constants::LOADING;

pub async fn load_async<F>(f: F) -> queries::TableResult
where
    F: FnOnce() -> queries::TableResult + Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| DbError::Pool(e.to_string()))?
}

pub fn sqlid_as_text(result: &Option<queries::TableResult>) -> String {
    match result {
        Some(Err(err)) => return format!("Error: {}", err),
        Some(Ok(data)) => format!("{}", data.get(0).and_then(|v| v.get(1)).map(|s| s.as_str()).unwrap_or("Unknown SQL ID")),
        None => return LOADING.to_string(),   
    }
}