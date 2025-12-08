use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if worker_count <= 1 {
        let mut map = HashMap::new();
        for str in input {
            for char in str
                .trim()
                .to_ascii_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
            {
                map.entry(char).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        return map;
    } else {
        let map = Arc::new(Mutex::new(HashMap::new()));
        let mut handles = vec![];

        for chunk in input.chunks(worker_count) {
            let chunk: Vec<String> = chunk.iter().map(|s| s.to_string()).collect();
            let map = Arc::clone(&map);

            let handle = thread::spawn(move || {
                for str in chunk {
                    for char in str
                        .trim()
                        .to_ascii_lowercase()
                        .chars()
                        .filter(|c| c.is_alphabetic())
                    {
                        map.lock()
                            .unwrap()
                            .entry(char)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        return Arc::try_unwrap(map).unwrap().into_inner().unwrap();
    }
}
