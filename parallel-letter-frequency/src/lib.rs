use std::collections::HashMap;
use std::thread;
use std::cmp;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        let slice_size = (input.len() + worker_count - 1) / worker_count;
        if slice_size == 0 {
            return HashMap::new();
        }
        let num_slices = (input.len() + slice_size - 1) / slice_size;
        let results = (0..num_slices).map(|i| {
            s.spawn(move || {
                let mut map = HashMap::new();
                for line in input[i * slice_size..cmp::min((i + 1) * slice_size, input.len())].iter() {
                    for chr in line.chars().filter(|c| c.is_alphabetic()) {
                        if let Some(c) = chr.to_lowercase().next() {
                            *map.entry(c).or_insert(0) += 1;
                        }
                    }
                }
                map
            })
        });

        let mut map = HashMap::new();
        for result in results {
            for (key, value) in result.join().unwrap() {
                *map.entry(key).or_insert(0) += value;
            }
        }
        map
    })
}
