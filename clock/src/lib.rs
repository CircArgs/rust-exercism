use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

/// a function with the same behavior as integer division in python e.g.
/// >>> -40//60
/// -1
/// >>> -60//60
/// -1
/// >>> -80//60
/// -2
fn python_style_integer_division(dividend: i32, divisor: u32) -> i32 {
    let divisor = divisor as i32;
    let ret = dividend / divisor;
    if dividend < 0 && dividend % divisor != 0 {
        return ret - 1;
    }
    ret
}

/// wrap-around modulus
/// takes a number, calculates the modulus and ensures it belongs to $Z_mod$
fn positive_modulus(dividend: i32, modulus: u32) -> i32 {
    let modulus = modulus as i32;
    let mut ret = dividend % modulus;
    if ret < 0 {
        ret += modulus;
    }
    ret
}

/// everything comes back to minutes
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::from_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::from_minutes(self.hours * 60 + self.minutes + minutes)
    }
    fn from_minutes(minutes: i32) -> Self {
        let quotient = python_style_integer_division(minutes, 60);
        let remainder = positive_modulus(minutes, 60);
        Clock {
            hours: positive_modulus(quotient, 24),
            minutes: remainder,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
