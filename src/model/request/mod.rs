use serde::Deserialize;

/// 分页列表参数
#[derive(Deserialize)]
pub struct ListOption {
    pub limit: i64,
    pub page: i64,
}

impl ListOption {
    pub fn apply_default(mut self) -> Self {
        self.limit = {
            if self.limit == 0 {
                10
            } else {
                self.limit
            }
        };
        self.page = {
            if self.page == 0 {
                1
            } else {
                self.page
            }
        };
        self
    }
}
