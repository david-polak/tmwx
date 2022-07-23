use std::collections::HashMap;
// use utils::get_epoch;

#[derive(Debug)]
struct CacheRecord {
    timestamp: u64,
    data: String,
}

pub struct Cache {
    taf: HashMap<String, CacheRecord>,
    metar: HashMap<String, CacheRecord>,
}

impl Cache {
    // let taf = HashMap::new();
    //

    pub fn new() -> Self {
        Self {
            taf: HashMap::new(),
            metar: HashMap::new(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn () {
//         println!("{}", get_epoch());
//     }
// }
