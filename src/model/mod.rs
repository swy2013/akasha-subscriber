pub mod proxy;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubscriptionUserinfo {
    pub upload: isize,
    pub download: isize,
    pub total: isize,
    pub expire: isize,
}

#[derive(Serialize, Deserialize)]
pub struct Redirect {
    pub domain: String,
    pub status_code: u16,
}
impl Redirect {
    pub fn new(domain: String, status_code: u16) -> Self {
        Self {
            domain,
            status_code,
        }
    }
}
