use super::{DbConfig,DbError};
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;
use std::sync::OnceLock;

static POOL: OnceLock<Result<Pool<OracleConnectionManager>, DbError>> = OnceLock::new();

fn init_pool() -> Result<Pool<OracleConnectionManager>, DbError> {
    // Load DB config
    let cfg = DbConfig::load_from_file("db_config.toml")?;

    let manager = OracleConnectionManager::new(
        &cfg.username,
        &cfg.password,
        &cfg.connect_string,
    );

    let pool = Pool::builder()
        .max_size(5)
        .build(manager)?;

    Ok(pool)
}

pub fn get_pool() -> Result<&'static Pool<OracleConnectionManager>, DbError> {
    let result_ref = POOL.get_or_init(|| init_pool());
    match result_ref {
        Ok(pool) => Ok(pool),
        Err(err) => Err(err.clone()), // return the stored error
    }
}
