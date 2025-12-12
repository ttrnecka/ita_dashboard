use thiserror::Error;

#[derive(Debug, Error,Clone)]
pub enum DbError {
    #[error("Pool error: {0}")]
    Pool(String),

    #[error("Oracle error: {0}")]
    Oracle(String),

    #[error("Config error: {0}")]
    Config(String),
}

impl From<r2d2::Error> for DbError {
    fn from(e: r2d2::Error) -> Self {
        DbError::Pool(e.to_string())
    }
}

impl From<r2d2_oracle::oracle::Error> for DbError {
    fn from(e: r2d2_oracle::oracle::Error) -> Self {
        DbError::Oracle(e.to_string())
    }
}

impl From<anyhow::Error> for DbError {
    fn from(e: anyhow::Error) -> Self {
        DbError::Config(e.to_string())
    }
}