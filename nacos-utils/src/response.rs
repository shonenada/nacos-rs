use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NacosResponse<T> {
    code: u16,
    message: Option<String>,
    data: T,
}

impl<T> NacosResponse<T> {
    pub fn get_data(self) -> T {
        self.data
    }
}
