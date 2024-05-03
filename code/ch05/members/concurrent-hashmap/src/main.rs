use concurrent_hashmap::ConcurrentHashMap;

fn main() {
    let map:ConcurrentHashMap<i32,i32> = ConcurrentHashMap::new();

    // Spawn some threads that concurrently insert and get values from the map.
    for i in 0..10 {
        let map = map.clone();
        std::thread::spawn(move || {
            map.insert(i, i * 2);
            let value = map.get(&i);
            println!("{:?}", value);
        });
    }
}
