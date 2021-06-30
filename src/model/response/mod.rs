use serde::{Deserialize, Serialize};

/// 接口响应元数据
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Meta {
    pub current_page: i64,
    pub total_page: i64,
    pub limit: i64,
    pub total: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Page<T> {
    pub record: Vec<T>,
    pub meta: Meta,
}

impl<T> Page<T> {
    pub fn build(record: Vec<T>, limit: i64, offset: i64, total: i64) -> Page<T> {
        Page {
            record,
            meta: Meta {
                current_page: (offset as f64 / limit as f64).ceil() as i64,
                total_page: (total as f64 / limit as f64).ceil() as i64,
                limit,
                total,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}
