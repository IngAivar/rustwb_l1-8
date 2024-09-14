// cargo run dash для запуска 

use std::thread;
use dashmap::DashMap;
use std::sync::Arc;

fn main() {
    // Используем Arc, чтобы делить карту между потоками
    let map = Arc::new(DashMap::new());

    let mut handles = vec![];

    for i in 0..10 {
        // Клонируем Arc<DashMap> для передачи в поток
        let map = Arc::clone(&map);

        let handle = thread::spawn(move || {
            // Вставка значения в DashMap
            map.insert(i, i * 2);
        });

        handles.push(handle);
    }

    // Ждем завершения всех потоков
    for handle in handles {
        handle.join().unwrap();
    }

    // Итерация и вывод всех элементов из DashMap
    for entry in map.iter() {
        println!("Key: {:#?}, Value: {:#?}", entry.key(), entry.value());
    }
}