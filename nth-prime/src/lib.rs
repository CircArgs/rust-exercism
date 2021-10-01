/// uses the sieve of eratosthenes up to a generous guess upper limit from the prime number theorem
pub fn nth(n: u32) -> u32 {
    let limit = upper_limit(n + 1);

    let mut ret: Vec<Option<bool>> = vec![None; limit];
    let mut tot_marked: u32 = 0;
    for e in 2..((limit as f64).sqrt() as usize + 1) {
        match ret[e - 2] {
            None => {
                ret[e - 2] = Some(true);

                if tot_marked == n {
                    return e as u32;
                }
                tot_marked += 1;
                let mut curr = e.pow(2);
                loop {
                    if curr > limit {
                        break;
                    }
                    ret[curr - 2] = Some(false);
                    curr += e;
                }
            }
            _ => continue,
        }
    }
    for e in ((limit as f64).sqrt() as usize + 1)..ret.len() {
        if tot_marked == (n + 1) {
            return (e - 1) as u32;
        }

        if ret[e - 2].is_none() {
            tot_marked += 1;
        }
    }
    0
}

fn upper_limit(n: u32) -> usize {
    ((((n as f64) * (n as f64).ln()) as usize) * 2).max(1000)
}
