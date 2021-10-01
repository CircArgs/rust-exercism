use counter::Counter;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::thread;

fn count_lower_chars(strings: Vec<String>) -> Counter<char> {
    let mut ret = Counter::new();
    strings
        .iter()
        .map(|s| {
            s.chars()
                .filter(|c| !(c.is_ascii_digit() || c.is_ascii_punctuation()))
                .map(|c| {
                    if c.is_ascii_uppercase() {
                        c.to_ascii_lowercase()
                    } else {
                        c
                    }
                })
                .collect::<Counter<char>>()
        })
        .for_each(|c| ret.extend(&c));

    ret
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = input.len() / worker_count + 1;
    let mut ret = Counter::new();
    let mut threads = input
        .chunks(chunk_size)
        .map(|ss| {
            let ss = ss.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            move || thread::spawn(move || count_lower_chars(ss))
        })
        .collect::<Vec<_>>();
    // work queue
    let mut active = VecDeque::new();
    loop {
        // no work left
        if threads.is_empty() && active.is_empty() {
            break;
        }
        // there's work and there are workers
        if !threads.is_empty() && (active.len() < worker_count) {
            // start working on another chunk
            active.push_back(threads.pop().unwrap()());
        }
        // there's no work and workers are working
        if threads.is_empty() || (active.len() == worker_count) {
            // wait for a worker to finish
            ret.extend(&active.pop_front().unwrap().join().unwrap());
        }
    }
    ret.into_map()
}
