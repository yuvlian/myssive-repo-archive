use chrono::Utc;

pub fn unix_timestamp_secs() -> u32 {
    Utc::now().timestamp().try_into().expect("u32 overflow")
}
