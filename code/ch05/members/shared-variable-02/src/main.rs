use std::sync::{Arc, RwLock};

fn main() {
    let rwlock = Arc::new(RwLock::new(0));

    // Spawn some threads that concurrently read the shared variable.
    let mut handles = vec![];
    let output = Arc::new(RwLock::new(String::new())); // Переменная для аккумулирования информации
    for _ in 0..10 {
        let rwlock = Arc::clone(&rwlock);
        let output = Arc::clone(&output);
        let handle = std::thread::spawn(move || {
            let mut value = rwlock.write().unwrap();
            *value += 1;

            print!("{}, ", *value);
            let thread_info = format!("{}, ", *value);
            let mut info = output.write().unwrap();
            *info += &thread_info;
        });
        handles.push(handle);
    }

    // Wait for all the threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
    println!();

    // Обработка накопленной информации
    let mut info = output.write().unwrap();
    // Удаление последних символов, пока они равны пробелу или запятой
    while info.ends_with(" ") || info.ends_with(",") {
        info.pop();
    }

    // Вывод накопленной информации на консоль
    println!("{info}");
}
