use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonResponse<T> {
    code: u16,
    message: Option<String>,
    data: T,
}

impl<T> CommonResponse<T> {
    pub fn get_data(self) -> T {
        self.data
    }
}
