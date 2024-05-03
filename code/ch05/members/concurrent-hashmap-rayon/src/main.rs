use rayon::prelude::*;
use std::sync::Arc;

use concurrent_hashmap_rayon::ConcurrentHashMap;

fn main() {
    let map = Arc::new(ConcurrentHashMap::new());

    (0..10).into_par_iter().for_each_with(map.clone(), |map, i| {
        map.insert(i, i * 2);
    });

    (0..10).into_par_iter().for_each_with(map.clone(), |map, i| {
        let value = map.get(&i);
        println!("{:?}", value);
    });
}