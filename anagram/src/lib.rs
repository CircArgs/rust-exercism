use counter::Counter;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = to_lower_chars(word);
    let counted = word.iter().collect::<Counter<_>>();
    possible_anagrams
        .into_iter()
        .filter(|s| {
            let tmp = to_lower_chars(s);
            tmp.iter().collect::<Counter<_>>() == counted && tmp != word
        })
        .map(|s| *s)
        .collect::<HashSet<_>>()
}

// with custom lowercase:
// Benchmark #1: cargo test -- --ignored
//   Time (mean ± σ):     193.5 ms ±   6.7 ms    [User: 167.9 ms, System: 23.5 ms]
//   Range (min … max):   185.1 ms … 209.1 ms    15 runs

// without:
// Benchmark #1: cargo test -- --ignored
//   Time (mean ± σ):     208.7 ms ±  25.0 ms    [User: 178.4 ms, System: 26.4 ms]
//   Range (min … max):   190.8 ms … 279.9 ms    15 runs

fn to_lower_chars(word: &str) -> Vec<char> {
    word.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().nth(0).unwrap()
            } else {
                c
            }
        })
        .collect::<Vec<char>>()
}
