// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(Debug, PartialEq, Eq)]
pub struct Duration {
    // storing seconds will be more precise
    s: u64,
}

impl Duration {
    /// get the earth years a duration represents
    pub fn to_years(&self) -> f64 {
        (self.s as f64) / 31557600.0
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}

pub trait Planet {
    const earth_years: f64;
    fn years_during(d: &Duration) -> f64 {
        d.to_years() / Self::earth_years
    }
}

macro_rules! planet {

    ($(,)* $p:ident; $y: expr $(,)*) => {
        pub struct $p;
        impl Planet for $p{
            const earth_years: f64 = $y;
        }
    };

    ($(,)* $p:ident; $y: expr $(, $pp:ident; $yp: expr)+) => {
        pub struct $p;
        impl Planet for $p{
            const earth_years: f64 = $y;
        }
        planet!($(, $pp; $yp)+);
    };
}

planet!(Mercury; 0.2408467,
Venus; 0.61519726,
Earth; 1.0,
Mars; 1.8808158,
Jupiter; 11.862615,
Saturn; 29.447498,
Uranus; 84.016846,
Neptune; 164.79132);
