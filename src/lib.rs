#[cfg(feature = "serde_support")]
extern crate serde;
#[cfg_attr(feature = "serde_support", macro_use)]
#[cfg(feature = "serde_support")]
extern crate serde_derive;
#[cfg(feature = "serde_support")]
extern crate serde_json;

#[macro_use]
extern crate error_chain;

#[cfg(feature = "serde_support")]
pub mod helpers;
#[cfg(feature = "serde_support")]
pub use helpers::*;

mod errors {
    #[cfg(feature = "serde_support")]
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            SerdeJsonError(::serde_json::Error);
            ParseFloatError(::std::num::ParseFloatError);
        }
    }
    #[cfg(not(feature = "serde_support"))]
    error_chain!{}
}
mod conrec;
pub use conrec::conrec;

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Segment {
    p1: Point,
    p2: Point,
    level: f64,
}

impl Segment {
    pub fn new(p1: Point, p2: Point, level: f64) -> Self {
        Segment {
            p1: p1,
            p2: p2,
            level: level,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn coord(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}
