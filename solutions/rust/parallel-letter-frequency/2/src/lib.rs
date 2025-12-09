use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        let mut handles = vec![];
        for chunk in input.chunks(input.len() / worker_count + 1) {
            let handle = s.spawn(move || {
                let mut local = HashMap::new();
                for line in chunk {
                    for c in line
                        .trim()
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphabetic())
                    {
                        *local.entry(c).and_modify(|e| *e+=1 as usize).or_insert(1);
                    }
                }
                local
            });
            handles.push(handle);
        }

        let mut final_map = HashMap::new();
        for handle in handles {
            for (k, v) in handle.join().unwrap().into_iter() {
                *final_map.entry(k).or_insert(0) += v;
            }
        }
        final_map
    })
}
