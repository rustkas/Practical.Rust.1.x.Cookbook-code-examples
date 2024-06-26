#[cfg(debug_assertions)]
fn main() {
    println!("Debugging mode enabled!");
    // Этот код будет включен только во время компиляции в режиме отладки
    // Например, можно добавить здесь проверки, логирование и т. д.
}

#[cfg(not(debug_assertions))]
fn main() {
    println!("Debugging mode disabled!");
    // Этот код будет включен при компиляции без режима отладки
    // Это может быть продакшн код или код, который не нужен при отладке
}