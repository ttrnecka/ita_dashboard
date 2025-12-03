use oracle::{Connection, Error};

pub fn fetch_oracle_data() -> Result<Vec<f32>, Error> {
    let conn = Connection::connect("user", "password", "1.1.1.1:1521/db")?;

    let sql = r#"
        SELECT 100 * used_gb / nullif(total_gb,0) 
        FROM dxc_v_temp_log
        WHERE log_date > SYSDATE - 1
        ORDER BY log_date
    "#;

    let mut values = Vec::new();

    for row_result in conn.query(sql, &[])? {
        let row = row_result?;
        let val: f32 = row.get(0)?;
        values.push(val);
    }

    Ok(values)
}
