use crate::PAGE_SIZE;
use serde::{Deserialize, Serialize};

fn default_page_size() -> i64 {
    PAGE_SIZE
}

/// Struct used to deserialize query strings.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct QuerySearch {
    pub q: Option<String>,
    pub sort: Option<Vec<String>>,
    #[serde(default)]
    pub offset: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

/// Struct used to serialize and deserialize paginated results.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub offset: i64,
    pub page_size: i64,
    pub total: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

impl<T> From<Vec<T>> for Page<T> {
    fn from(vec: Vec<T>) -> Self {
        let len: i64 = vec.len() as i64;
        Page {
            data: vec,
            offset: 0,
            page_size: len,
            total: len,
            message: None,
            warning: None,
        }
    }
}

impl<T> Page<T> {
    pub fn empty() -> Page<T> {
        Page {
            data: Vec::new(),
            offset: 0,
            page_size: 0,
            total: 0,
            message: None,
            warning: None,
        }
    }

    pub fn with_data(data: Vec<T>, total: i64, offset: i64) -> Self {
        let page_size: i64 = data.len() as i64;
        Page {
            data,
            total,
            offset,
            page_size,
            message: None,
            warning: None,
        }
    }
}
