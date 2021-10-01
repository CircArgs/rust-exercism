use lazy_static::lazy_static;
use std::collections::HashSet;
lazy_static! {
    static ref CACHE: Vec<HashSet<[u32; 3]>> = {
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(find(i));
        }
        v
    };
}

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut ret = HashSet::new();
    if sum < CACHE.len() as u32 {
        ret.union(&CACHE[sum as usize]);
        ret
    } else if sum % 2 == 1 {
        ret
    } else {
        match CACHE
            .iter()
            .enumerate()
            .rev()
            .find(|(i, _)| sum % (*i as u32) == 0)
        {
            Some((_, hs)) => {
                ret.union(hs);
            }
            _ => {}
        };

        let sum = sum / 2;
        let mut step = 1;
        if sum % 2 == 1 {
            step = 2;
        }

        for m in ((CACHE.len() as u32)..((step as f64).sqrt() as u32))
            .step_by(step)
            .filter(|i| sum % i == 0)
        {
            let n = m - sum / m;
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            if a > b {
                ret.insert([b, a, c]);
            } else {
                ret.insert([a, b, c]);
            }
        }
        ret
    }
}
