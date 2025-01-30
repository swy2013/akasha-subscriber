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
