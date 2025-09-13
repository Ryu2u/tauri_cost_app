use crate::error::Exception::{RuntimeException, SqlException};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub enum Exception {
    RuntimeException(String),
    SqlException(String),
    NotFount,
}

impl Display for Exception {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeException(e) => write!(f, "RuntimeException: {}", e),
            SqlException(e) => write!(f, "SqlException: {}", e),
            NotFound => write!(f, "NotFound"),
        }
    }
}
