use lazy_static::lazy_static;
use std::char;
use std::collections::HashMap;
lazy_static! {
    static ref CHARMAP: HashMap<(char, usize), u32> = {
        let mut m = HashMap::new();
        for i in 0..10 {
            m.insert((char::from_digit(i, 10).unwrap(), 0), i);
            let mut temp = 2 * i;
            if temp > 9 {
                temp -= 9;
            }
            m.insert((char::from_digit(i, 10).unwrap(), 1), temp);
        }
        m
    };
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() < 2 || code == " 0" {
        return false;
    }
    let mut tot = 0;
    for r in code
        .chars()
        .rev()
        .filter(|c| *c != ' ')
        .enumerate()
        .map(|(i, c)| CHARMAP.get(&(c, i % 2)).map_or(Err("whoops"), |v| Ok(v)))
    {
        match r {
            Ok(n) => tot += n,
            _ => return false,
        }
    }

    println!("{}", tot);
    tot % 10 == 0
}
