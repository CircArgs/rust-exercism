Thanks for the suggested solution. I had the following thoughts about my original solution before restarting from scratch:

1. my chunks block via `join` until all threads are complete per chunk
2. the locking is an obvious issue but doesn't seem to impact performance as much as 1
3. I really just want to queue threads up and dequeue new threads when the number of active threads is less than the number of worker threads given, but I stumbled in to the chunking first

I implemented 3 and saw almost no increase in performance:

```rust
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut ret = Counter::new();
    let mut threads = input
        .iter()
        .map(|s| {
            let s = s.to_string();
            move || thread::spawn(move || count_lower_chars(s))
        })
        .collect::<Vec<_>>();
    // work queue
    let mut active = VecDeque::new();
    loop {
        // no work left
        if threads.is_empty() && active.is_empty() {
            break;
        }
        if !threads.is_empty() && (active.len() < worker_count) {
            active.push_back(threads.pop().unwrap()());
        }
        if threads.is_empty() || (active.len() == worker_count) {
            ret.extend(&active.pop_front().unwrap().join().unwrap());
        }
    }
    ret.into_map()
}
```

// test bench_large_parallel ... bench: 30,778,200 ns/iter (+/- 1,893,899)
// test bench_large_sequential ... bench: 899,721 ns/iter (+/- 138,929)
// test bench_small_parallel ... bench: 1,371,107 ns/iter (+/- 1,029,964)
// test bench_small_sequential ... bench: 31,096 ns/iter (+/- 3,634)
// test bench_tiny_parallel ... bench: 126,178 ns/iter (+/- 73,452)
// test bench_tiny_sequential ... bench: 81 ns/iter (+/- 11)

then I realized the issue was probably that most of the time was spent creating threads not doing real work. I looked back at your reference solution and realized exactly that. So, implemented the chunking per thread and got

// test bench_large_parallel ... bench: 696,622 ns/iter (+/- 57,923)
// test bench_large_sequential ... bench: 906,752 ns/iter (+/- 78,498)
// test bench_small_parallel ... bench: 212,646 ns/iter (+/- 24,612)
// test bench_small_sequential ... bench: 36,517 ns/iter (+/- 17,922)
// test bench_tiny_parallel ... bench: 122,763 ns/iter (+/- 52,255)
// test bench_tiny_sequential ... bench: 87 ns/iter (+/- 22)

`jguinart's solution` on my machine:

// test bench_large_parallel ... bench: 453,907 ns/iter (+/- 38,026)
// test bench_large_sequential ... bench: 917,498 ns/iter (+/- 491,874)
// test bench_small_parallel ... bench: 17,453 ns/iter (+/- 10,263)
// test bench_small_sequential ... bench: 31,051 ns/iter (+/- 2,455)
// test bench_tiny_parallel ... bench: 59 ns/iter (+/- 22)
// test bench_tiny_sequential ... bench: 79 ns/iter (+/- 7)

the major differences now I think are:

- I didn't do any basic checks that would particularly accelerate small tests

- noticing the similarities between jguinart's `chunk.iter().map` and mine as I said I imitated that chunking per thread once I realized it I also noticed jguinart's looks to instantiate all
  threads upon `.collect::<Vec<_>>()`. If I'm right that would mean `worker_count` is mostly ignored by jguinart's solution and it simply spawns the number of threads the os will give it. I noticed this because while writing my solution that did not chunk per thread I figured I would have to make sure a threads were still lazily spawned even after `collect` so I wrapped each spawn in a closure

- finally, switching to the same counting logic as jguinart's solution i.e. using `HashMap` I get:
  //test bench_large_parallel ... bench: 451,668 ns/iter (+/- 77,768)
  //test bench_large_sequential ... bench: 918,438 ns/iter (+/- 98,630)
  //test bench_small_parallel ... bench: 216,031 ns/iter (+/- 87,816)
  //test bench_small_sequential ... bench: 31,061 ns/iter (+/- 2,415)
  //test bench_tiny_parallel ... bench: 125,555 ns/iter (+/- 38,071)
  //test bench_tiny_sequential ... bench: 81 ns/iter (+/- 9)

I still doubt `Counter` is responsible for this as it just wraps `HashMap`. I think the discrepancy is more down to my `count_lower_chars`. I've just submitted my own solution adapted to my realization because I'm a bum.
