// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_IN_SECONDS * Self::ORBITAL_PERIOD)
    }

    const ORBITAL_PERIOD: f64;
}

macro_rules! make_planet {
    ($name:ident, $orbital_period:expr) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $orbital_period;
        }
    };
}

make_planet!(Mercury, 0.2408467);
make_planet!(Venus, 0.61519726);
make_planet!(Earth, 1.0);
make_planet!(Mars, 1.8808158);
make_planet!(Jupiter, 11.862615);
make_planet!(Saturn, 29.447498);
make_planet!(Uranus, 84.016846);
make_planet!(Neptune, 164.79132);
