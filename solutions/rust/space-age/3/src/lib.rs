// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const MERCURY_ORBITAL_PERIOD: f64 = 0.2408467;
const VENUS_ORBITAL_PERIOD: f64 = 0.61519726;
const EARTH_ORBITAL_PERIOD: f64 = 1.0;
const MARS_ORBITAL_PERIOD: f64 = 1.8808158;
const JUPITER_ORBITAL_PERIOD: f64 = 11.862615;
const SATURN_ORBITAL_PERIOD: f64 = 29.447498;
const URANUS_ORBITAL_PERIOD: f64 = 84.016846;
const NEPTUNE_ORBITAL_PERIOD: f64 = 164.79132;

const ONE_EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    secs: u64
}

impl Duration {
    pub fn new(s: u64) -> Self {
        Duration { secs: s }
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration::new(s)
    }
}

pub trait Planet {
    fn orbital_period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.secs as f64 / ONE_EARTH_YEAR_IN_SECONDS / Self::orbital_period()
    }
}

macro_rules! planet {
    ($p:ident, $o:expr) => {
        pub struct $p;
        impl Planet for $p {
            fn orbital_period() -> f64 {
                $o
            }
        }
    };
}

planet!(Mercury, MERCURY_ORBITAL_PERIOD);
planet!(Venus, VENUS_ORBITAL_PERIOD);
planet!(Earth, EARTH_ORBITAL_PERIOD);
planet!(Mars, MARS_ORBITAL_PERIOD);
planet!(Jupiter, JUPITER_ORBITAL_PERIOD);
planet!(Saturn, SATURN_ORBITAL_PERIOD);
planet!(Uranus, URANUS_ORBITAL_PERIOD);
planet!(Neptune, NEPTUNE_ORBITAL_PERIOD);