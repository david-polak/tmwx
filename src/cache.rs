// use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheRecord {
    timestamp: u64,
    data: String,
}

// #[derive(Debug)]
// pub struct Cache {
//     taf: HashMap<String, CacheRecord>,
//     metar: HashMap<String, CacheRecord>,
// }
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
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_cache_1() {
//         // println!("{}", get_epoch());
//         let cache = Cache::new();
//         // let record =
//         // cache.taf.insert("LKPR", )
//     }
// }
