use super::{DbError, get_pool};
use chrono::NaiveDateTime;

#[derive(Debug,Clone)]
pub struct TempDataPoint {
    pub ts: NaiveDateTime,
    pub a: f32,
    pub b: f32,
}

pub fn fetch_temp_data() -> Result<Vec<TempDataPoint>, DbError> {
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

        values.push(TempDataPoint {
            ts,
            a: used as f32,
            b: total as f32,
        });
    }

    Ok(values)
}
// #[derive(Debug,Clone)]
// pub struct TableSpaceRow(pub String,pub u32,pub u32,pub u32,pub u32,pub u32);

pub fn fetch_tablespace_data() -> Result<Vec<Vec<String>>, DbError> {
    let pool = get_pool()?;
    let conn = pool.get()?;

    let sql = r#"
        SELECT 
            df.tablespace_name Tablespace,
            totalusedspace Used_MB,
            (df.totalspace - tu.totalusedspace) Free_MB,
            df.totalspace Total_MB,
            df.maxspace Max_Total_MB,
            ROUND(100 - 100 * ( (df.maxspace - tu.totalusedspace)/ df.maxspace)) Used_pct
        FROM
            (SELECT 
                tablespace_name,
                ROUND(SUM(bytes) / 1048576) TotalSpace,
                ROUND(SUM(greatest(maxbytes,bytes)) / 1048576) MaxSpace
            FROM dba_data_files
            GROUP BY tablespace_name
            ) df,
            (SELECT 
                ROUND(SUM(bytes)/(1024*1024)) totalusedspace,
                tablespace_name
            FROM dba_segments
            GROUP BY tablespace_name
            ) tu
        WHERE df.tablespace_name = tu.tablespace_name
        AND df.tablespace_name not like 'UNDO%'
        ORDER BY 6 desc
    "#;

    let mut values = Vec::new();

    for row_result in conn.query(sql, &[])? {
        let row = row_result?;   
        let name: String = row.get(0)?;
        let used: String = row.get(1)?;
        let free: String = row.get(2)?;
        let total: String = row.get(3)?;
        let total_max: String = row.get(4)?;
        let used_pct: String = row.get(5)?;

        values.push(vec!(name, used, free,total,total_max,used_pct));
    }

    Ok(values)
}