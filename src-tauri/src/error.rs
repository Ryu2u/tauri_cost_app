use crate::error::Exception::{IoException, RuntimeException, SqlException};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub enum Exception {
    RuntimeException(String),
    SqlException(String),
    IoException(String),
    NotFound,
}

impl Display for Exception {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeException(e) => write!(f, "RuntimeException: {}", e),
            SqlException(e) => write!(f, "SqlException: {}", e),
            IoException(e) => write!(f, "IoException: {}", e),
            Exception::NotFound => write!(f, "NotFound"),
        }
    }
}

impl std::error::Error for Exception {}

impl From<sqlx::Error> for Exception {
    fn from(e: sqlx::Error) -> Self {
        SqlException(e.to_string())
    }
}

impl From<std::io::Error> for Exception {
    fn from(e: std::io::Error) -> Self {
        IoException(e.to_string())
    }
}
