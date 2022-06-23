use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod destiny;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response<T> {
    detailed_error_trace: Option<String>,
    error_code: i32,
    error_status: String,
    message: String,
    message_data: HashMap<String, String>,
    response: T,
    throttle_seconds: i32,
}

impl<T> Response<T> {
    pub fn detailed_error_trace(&self) -> Option<&String> {
        self.detailed_error_trace.as_ref()
    }

    pub fn error_code(&self) -> i32 {
        self.error_code
    }

    pub fn error_status(&self) -> &str {
        self.error_status.as_ref()
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn message_data(&self) -> &HashMap<String, String> {
        &self.message_data
    }

    pub fn response(&self) -> &T {
        &self.response
    }

    pub fn throttle_seconds(&self) -> i32 {
        self.throttle_seconds
    }
}
