use super::{DbError, get_pool};
use chrono::NaiveDateTime;

#[derive(Debug,Clone)]
pub struct DataPoint {
    pub ts: NaiveDateTime,
    pub a: f32,
    pub b: f32,
}

pub fn fetch_temp_data() -> Result<Vec<DataPoint>, DbError> {
    let pool = get_pool()?;
    let conn = pool.get()?;

    let sql = r#"
        SELECT log_date, used_gb, total_gb 
        FROM dxc_v_temp_log
        WHERE log_date > SYSDATE - 7
        ORDER BY log_date
    "#;

    let mut values = Vec::new();

    for row_result in conn.query(sql, &[])? {
        let row = row_result?;   
        let ts: NaiveDateTime = row.get(0)?;
        let used: f64 = row.get(1)?;
        let total: f64 = row.get(2)?;

        values.push(DataPoint {
            ts,
            a: used as f32,
            b: total as f32,
        });
    }

    Ok(values)
}
