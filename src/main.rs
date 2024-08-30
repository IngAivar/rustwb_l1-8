use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use std::thread;
use std::time::Duration;

fn main() {
    // Создаем HashMap с Mutex
    let shared_data = Arc::new(Mutex::new(HashMap::new()));

    // Создаем DashMap
    let dash_map: DashMap<String, i32> = DashMap::new();

    // Функция для записи в HashMap с Mutex
    fn write_to_hashmap(data: Arc<Mutex<HashMap<String, i32>>>, key: String, value: i32) {
        let mut data = data.lock().unwrap();
        data.insert(key, value);
    }

    // Функция для записи в DashMap
    fn write_to_dashmap(data: &DashMap<String, i32>, key: String, value: i32) {
        data.insert(key, value);
    }

    // Создаем несколько потоков для записи
    let threads = 10;
    for i in 0..threads {
        let shared_data = shared_data.clone();
        let dash_map = dash_map.clone();
        thread::spawn(move || {
            for j in 0..1000 {
                write_to_hashmap(shared_data.clone(), format!("key_{}", i * 1000 + j), j);
                write_to_dashmap(&dash_map, format!("key_{}", i * 1000 + j), j);
            }
        });
    }

    // Ждем завершения всех потоков
    thread::sleep(Duration::from_secs(2));

    // Выводим результаты (для проверки)
    println!("HashMap with Mutex: {:?}", *shared_data.lock().unwrap());
    println!("DashMap: {:?}", dash_map);
}