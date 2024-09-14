use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    // Создаем обертку Arc<Mutex<HashMap>>, чтобы безопасно делить доступ к HashMap между потоками.
    let map = Arc::new(Mutex::new(HashMap::new()));

    // Вектор для хранения дескрипторов потоков.
    let mut handles = vec![];

    // Создаем 10 потоков, каждый из которых будет записывать данные в HashMap.
    for i in 0..10{

        // Клонируем Arc, чтобы передать его в новый поток.
        let map = Arc::clone(&map);

        // Создаем и запускаем поток.
        let handle = thread::spawn(move || {

            // Захватываем блокировку Mutex перед доступом к HashMap.
            let mut data = map.lock().unwrap();

            // Вставляем пару ключ-значение в HashMap.
            data.insert(i, i * 2);
        });

        // Добавляем дескриптор потока в вектор.
        handles.push(handle);
    }

    // Ждем завершения всех потоков.
    for handle in handles{
        handle.join().unwrap();
    }

    // После завершения всех потоков, захватываем блокировку для чтения данных.
    let data = map.lock().unwrap();

    // Печатаем содержимое HashMap.
    println!("{:#?}", *data);

}