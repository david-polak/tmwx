use directories::ProjectDirs;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn get_cache_dir() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "polakdavid", "tmwx") {
        PathBuf::from(proj_dirs.cache_dir())
    } else {
        panic!("Error: Could not find a suitable cache directory.")
    }
}

#[cfg(test)]
mod get_epoch_tests {
    use super::*;

    #[test]
    fn is_greater_than_zero() {
        assert!(get_epoch() > 0)
    }
}
