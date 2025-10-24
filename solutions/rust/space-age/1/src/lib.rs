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
        // todo!("s, measured in seconds: {s}");
        Duration::new(s)
    }
}

pub trait Planet {
    fn orbital_period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        // todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        d.secs as f64 / ONE_EARTH_YEAR_IN_SECONDS / Self::orbital_period()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_period() -> f64 {
        MERCURY_ORBITAL_PERIOD
    }
}
impl Planet for Venus {
    fn orbital_period() -> f64 {
        VENUS_ORBITAL_PERIOD
    }
}
impl Planet for Earth {
    fn orbital_period() -> f64 {
        EARTH_ORBITAL_PERIOD
    }
}
impl Planet for Mars {
    fn orbital_period() -> f64 {
        MARS_ORBITAL_PERIOD
    }
}
impl Planet for Jupiter {
    fn orbital_period() -> f64 {
        JUPITER_ORBITAL_PERIOD
    }
}
impl Planet for Saturn {
    fn orbital_period() -> f64 {
        SATURN_ORBITAL_PERIOD
    }
}
impl Planet for Uranus {
    fn orbital_period() -> f64 {
        URANUS_ORBITAL_PERIOD
    }
}
impl Planet for Neptune {
    fn orbital_period() -> f64 {
        NEPTUNE_ORBITAL_PERIOD
    }
}
