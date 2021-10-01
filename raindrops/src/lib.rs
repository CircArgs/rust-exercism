// The rules of raindrops are that if a given number:

// - has 3 as a factor, add 'Pling' to the result.
// - has 5 as a factor, add 'Plang' to the result.
// - has 7 as a factor, add 'Plong' to the result.
// - does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.

pub fn raindrops(n: u32) -> String {
    let mut ret = vec![];
    if n % 3 == 0 {
        ret.push("Pling".to_string());
    }
    if n % 5 == 0 {
        ret.push("Plang".to_string());
    }
    if n % 7 == 0 {
        ret.push("Plong".to_string());
    }
    if ret.len() == 0 {
        ret.push(format!("{}", n));
    }
    ret.join("")
}
