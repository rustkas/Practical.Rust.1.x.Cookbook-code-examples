use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ConcurrentQueue<T> {
    inner: Arc<Mutex<Vec<T>>>,
}

impl<T> ConcurrentQueue<T> {
    pub fn new() -> Self {
        ConcurrentQueue {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, value: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.push(value);
    }

    pub fn pop(&self) -> Option<T> {
        let mut inner = self.inner.lock().unwrap();
        inner.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop_single_thread() {
        let queue = ConcurrentQueue::new();
        queue.push(1);
        assert_eq!(queue.pop(), Some(1));
    }

    #[test]
    fn test_push_multiple_items() {
        let queue = Arc::new(ConcurrentQueue::new());
        let queue_clone1 = Arc::clone(&queue);
        let queue_clone2 = Arc::clone(&queue);
        let queue_clone3 = Arc::clone(&queue);

        let push_thread1 = std::thread::spawn(move || {
            queue_clone1.push(1);
        });

        let push_thread2 = std::thread::spawn(move || {
            queue_clone2.push(2);
        });

        let push_thread3 = std::thread::spawn(move || {
            queue_clone3.push(3);
        });

        push_thread1.join().unwrap();
        push_thread2.join().unwrap();
        push_thread3.join().unwrap();

        let mut popped_items = vec![];
        while let Some(item) = queue.pop() {
            popped_items.push(item);
        }

        assert_eq!(popped_items.len(), 3);
        assert!(popped_items.contains(&1));
        assert!(popped_items.contains(&2));
        assert!(popped_items.contains(&3));
    }

    #[test]
    fn test_pop_empty_queue() {
        let queue: ConcurrentQueue<i32> = ConcurrentQueue::new();
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_push_pop_concurrently() {
        let queue = ConcurrentQueue::new();
        let queue_clone = queue.clone();

        let push_thread = std::thread::spawn(move || {
            for i in 1..=100 {
                queue_clone.push(i);
            }
        });

        let pop_thread = std::thread::spawn(move || {
            for _ in 1..=100 {
                assert_ne!(queue.pop(), None);
            }
        });

        push_thread.join().unwrap();
        pop_thread.join().unwrap();
    }

    #[test]
    fn test_concurrent_pushes() {
        let queue = Arc::new(ConcurrentQueue::new());
        let queue_clone1 = Arc::clone(&queue);
        let queue_clone2 = Arc::clone(&queue);

        let push_thread1 = std::thread::spawn(move || {
            for i in 1..=50 {
                queue_clone1.push(i);
            }
        });

        let push_thread2 = std::thread::spawn(move || {
            for i in 51..=100 {
                queue_clone2.push(i);
            }
        });

        push_thread1.join().unwrap();
        push_thread2.join().unwrap();

        let mut popped_items = Vec::new();
        while let Some(item) = queue.pop() {
            popped_items.push(item);
        }

        // Сортируем извлеченные элементы, чтобы сравнить их с ожидаемым порядком
        popped_items.sort();

        // Создаем ожидаемый порядок элементов от 1 до 100
        let expected_order: Vec<_> = (1..=100).collect();

        // Проверяем, что извлеченные элементы соответствуют ожидаемому порядку
        assert_eq!(expected_order, popped_items);
    }

    #[test]
    fn test_push_pop_concurrently_with_delay() {
        let queue = ConcurrentQueue::new();
        let queue_clone = queue.clone();
    
        let push_thread = std::thread::spawn(move || {
            for i in 1..=50 {
                queue_clone.push(i);
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    
        let pop_thread = std::thread::spawn(move || {
            for _ in 1..=50 {
                loop {
                    if let Some(_) = queue.pop() {
                        // Если удалось извлечь элемент, то успешно
                        // продолжаем выполнение
                        break;
                    } else {
                        // Иначе ждем некоторое время и пытаемся
                        // извлечь элемент еще раз
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            }
        });
    
        push_thread.join().unwrap();
        pop_thread.join().unwrap();
    }
    

    #[test]
    fn test_push_pop_concurrently_with_delay_and_exceptions() {
        let queue = ConcurrentQueue::new();
        let queue_clone = queue.clone();
    
        let push_thread = std::thread::spawn(move || {
            for i in 1..=50 {
                queue_clone.push(i);
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    
        let pop_thread = std::thread::spawn(move || {
            for _ in 1..=50 {
                loop {
                    if let Some(_) = queue.pop() {
                        // Если удалось извлечь элемент, то успешно
                        // продолжаем выполнение
                        std::thread::sleep(std::time::Duration::from_millis(10));
                        break;
                    } else {
                        // Иначе ждем некоторое время и пытаемся
                        // извлечь элемент еще раз
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            }
        });
    
        push_thread.join().unwrap();
        pop_thread.join().unwrap();
    }
    
    #[test]
    fn test_push_pop_concurrently_with_delay_and_exceptions_reverse() {
        let queue = ConcurrentQueue::new();
        let queue_clone = queue.clone();
    
        let push_thread = std::thread::spawn(move || {
            for i in 1..=50 {
                queue_clone.push(i);
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    
        std::thread::sleep(std::time::Duration::from_millis(50)); // Задержка перед началом первого потока
    
        let pop_thread = std::thread::spawn(move || {
            for _ in 1..=50 {
                if let Some(_) = queue.pop() {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                } else {
                    assert!(false, "pop returned None unexpectedly");
                }
            }
        });
    
        push_thread.join().unwrap();
        pop_thread.join().unwrap();
    }
    
}
