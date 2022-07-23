use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod get_epoch_tests {
    use super::*;

    #[test]
    fn is_greater_than_zero() {
        assert!(get_epoch() > 0)
    }
}
