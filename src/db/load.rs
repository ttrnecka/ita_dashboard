use super::queries;
use super::DbError;

// pub type TableResult = Result<Vec<Vec<String>>, String>;

// pub fn default_table_result() -> TableResult {
//     Ok(Vec::new())
// }

pub async fn load_async<F>(f: F) -> queries::TableResult
where
    F: FnOnce() -> queries::TableResult + Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| DbError::Pool(e.to_string()))?
}

pub fn sqlid_as_text(result: &queries::TableResult) -> String {
    if let Err(err) = result {
        return format!("Error: {}", err);
    }
    if let Ok(data) = result {
        return format!("{}", data.get(0).and_then(|v| v.get(1)).map(|s| s.as_str()).unwrap_or("Unknown SQL ID"));
    }
    "No data".to_string()
}