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

pub type TableResult = Result<Vec<Vec<String>>, DbError>;

pub fn fetch_tablespace_data() -> TableResult {
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

// Fetch filesytem utilization data
pub fn fetch_filesystem_data() -> TableResult {
    let pool = get_pool()?;
    let conn = pool.get()?;

    let sql = r#"
        SELECT
            filesystem,
            size_1,
            used,
            available,
            to_number(regexp_replace(use, '%', '')) use,
            mounted
        FROM
            ext_v_db_filesystem
        ORDER BY mounted
    "#;

    let mut values = Vec::new();

    for row_result in conn.query(sql, &[])? {
        let row = row_result?;   
        let fs: String = row.get(0)?;
        let size: String = row.get(1)?;
        let used: String = row.get(2)?;
        let available: String = row.get(3)?;
        let used_pct: String = row.get(4)?;
        let mounted: String = row.get(5)?;

        values.push(vec!(fs, size, used, available, used_pct, mounted));
    }

    Ok(values)
}

// pub fn fetch_session_history_data() -> Result<Vec<Vec<String>>, DbError> {
pub fn fetch_session_history_data(start_date: NaiveDateTime, end_date: NaiveDateTime) -> TableResult {
    // let start_date = NaiveDateTime::parse_from_str("2026-04-14 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    // let end_date = NaiveDateTime::parse_from_str("2026-04-14 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
    let pool = get_pool()?;
    let conn = pool.get()?;

    let sql = r#"
    SELECT 
        ASH.session_id sid,
        ASH.session_serial# serial#,
        ASH.sql_id,
        ASH.sql_opname,
        MIN(sample_time) sql_start_time,
        MAX(sample_time) sql_end_time,
        ROUND(((CAST(MAX(sample_time) AS DATE)) - (CAST(MIN(sample_time) AS DATE))) * (3600*24),0) etime_secs ,
        ROUND(((CAST(MAX(sample_time) AS DATE)) - (CAST(MIN(sample_time) AS DATE))) * (60*24),1) etime_mins ,
        MAX(temp_space_allocated)/(1024*1024) max_temp_mb
        FROM DBA_HIST_ACTIVE_SESS_HISTORY ASH
        WHERE ASH.session_type = 'FOREGROUND'
        AND ASH.sql_id        IS NOT NULL
        AND sample_time BETWEEN :start_date AND :end_date
        GROUP BY ASH.instance_number,
        ASH.user_id,
        ASH.session_id,
        ASH.session_serial#,
        ASH.sql_id,
        ASH.sql_opname,
        ASH.sql_exec_id,
        ASH.module
        HAVING MAX(temp_space_allocated) > 0 AND
        ROUND(((CAST(MAX(sample_time) AS DATE)) - (CAST(MIN(sample_time) AS DATE))) * (3600*24),0) > 0
        ORDER BY MAX(temp_space_allocated) DESC
        FETCH FIRST 100 ROWS ONLY
    "#;

    let mut values = Vec::new();

    for row_result in conn.query(sql, &[&start_date, &end_date])? {
    // for row_result in conn.query(sql, &[])? {
        let row = row_result?;   
        let sid: String = row.get::<usize, Option<String>>(0)?.unwrap_or_default();
        let serial: String = row.get::<usize, Option<String>>(1)?.unwrap_or_default();
        let sql_id: String = row.get::<usize, Option<String>>(2)?.unwrap_or_default();
        let sql_opname: String = row.get::<usize, Option<String>>(3)?.unwrap_or_default();
        let sql_start_time: String = row.get::<usize, Option<String>>(4)?.unwrap_or_default();
        let sql_end_time: String = row.get::<usize, Option<String>>(5)?.unwrap_or_default();
        let etime_secs: String = row.get::<usize, Option<String>>(6)?.unwrap_or_default();
        let etime_mins: String = row.get::<usize, Option<String>>(7)?.unwrap_or_default();
        let max_temp_mb: String = row.get::<usize, Option<String>>(8)?.unwrap_or_default();

        values.push(vec!(sid, serial, sql_id, sql_opname, sql_start_time, sql_end_time, etime_secs, etime_mins, max_temp_mb));
    }

    Ok(values)
}

pub fn fetch_session_temp_data() -> TableResult {
    let pool = get_pool()?;
    let conn = pool.get()?;

    let sql = r#"
        SELECT
            s.sid,
            s.serial#,
            s.username,
            s.program,
            u.tablespace,
            u.segtype,
            u.blocks * t.block_size / 1024 / 1024 AS temp_mb_used,
            s.sql_id,
            q.sql_text
        FROM
            v$tempseg_usage u
            JOIN v$session s ON s.saddr = u.session_addr
            JOIN dba_tablespaces t ON u.tablespace = t.tablespace_name
            LEFT JOIN v$sql q ON s.sql_id = q.sql_id
        ORDER BY
            temp_mb_used DESC
    "#;

    let mut values = Vec::new();

    for row_result in conn.query(sql, &[])? {
    // for row_result in conn.query(sql, &[])? {
        let row = row_result?;   
        let sid: String = row.get::<usize, Option<String>>(0)?.unwrap_or_default();
        let serial: String = row.get::<usize, Option<String>>(1)?.unwrap_or_default();
        let username: String = row.get::<usize, Option<String>>(2)?.unwrap_or_default();
        let program: String = row.get::<usize, Option<String>>(3)?.unwrap_or_default();
        let tablespace: String = row.get::<usize, Option<String>>(4)?.unwrap_or_default();
        let segtype: String = row.get::<usize, Option<String>>(5)?.unwrap_or_default();
        let temp_mb_used: String = row.get::<usize, Option<String>>(6)?.unwrap_or_default();
        let sql_id: String = row.get::<usize, Option<String>>(7)?.unwrap_or_default();
        let sql_text: String = row.get::<usize, Option<String>>(8)?
            .unwrap_or_default()
            .chars()
            .take(47)
            .collect::<String>()
            + if row.get::<usize, Option<String>>(8)?.unwrap_or_default().len() > 50 { "..." } else { "" };

        values.push(vec!(sid, serial, username, program, tablespace, segtype, temp_mb_used, sql_id, sql_text));
    }

    Ok(values)
}

pub fn fetch_sqlid_data(sql_id: &str) -> TableResult {
    let pool = get_pool()?;
    let conn = pool.get()?;

    let sql = r#"
        SELECT sql_id,sql_text
        FROM DBA_HIST_SQLTEXT
        WHERE sql_id = :sql_id
        UNION ALL
        SELECT sql_id,sql_fulltext
        FROM gv$sql
        WHERE sql_id = :sql_id
    "#;

    let mut values = Vec::new();

    for row_result in conn.query(sql, &[&sql_id])? {
        let row = row_result?;   
        let sql_id: String = row.get(0)?;
        let sql_text: String = row.get(1)?;


        values.push(vec!(sql_id, sql_text));
    }

    Ok(values)
}