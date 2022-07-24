use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::get_epoch;

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheRecord {
    timestamp: u64,
    data: String,
}

#[derive(Debug)]
pub struct Cache {
    map: HashMap<String, CacheRecord>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, icao: String, value: String) {
        let record = CacheRecord {
            timestamp: get_epoch(),
            data: value,
        };

        self.map.insert(icao, record);
    }

    pub fn get(&self, icao: &String) -> Option<&CacheRecord> {
        self.map.get(icao)
    }
}

//
// impl Cache {
//     // let taf = HashMap::new();
//
//     pub fn new() -> Self {
//         Self {
//             taf: HashMap::new(),
//             metar: HashMap::new(),
//         }
//     }
// }
//
#[cfg(test)]
mod cache_tests {
    use super::*;

    #[test]
    fn test_cache_inserts_records() {
        let mut cache = Cache::new();
        let expected1 = String::from("expected data 1");
        let icao1 = String::from("LKPR");

        let expected2 = String::from("expected data 2");
        let icao2 = String::from("LKHK");

        cache.insert(icao1.clone(), expected1.clone());
        cache.insert(icao2.clone(), expected2.clone());

        assert!(Some(cache.map.get(&icao1)).is_some());
        assert!(Some(cache.map.get(&icao2)).is_some());
    }
}
