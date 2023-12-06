// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    pub fn new(seconds: u64) -> Self {
        Duration { seconds }
    }
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration::new(seconds)
    }
}



pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

macro_rules! planet {
    ($planet:ident, $earth_years:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                (d.seconds as f64) / ($earth_years * 31557600.0)
            }
        }
    };
}


planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

