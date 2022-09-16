use std::time::{SystemTime, UNIX_EPOCH};
use md5::{Md5, Digest};

pub fn gen_id() -> String {
    // Get current timestamp with miliseconds
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let hash = Md5::digest(timestamp.to_string());

    format!("{:x}", hash)[..=5].to_string()
}
