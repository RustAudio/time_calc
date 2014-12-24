//!
//!  division.rs
//!
//!  Created by Mitchell Nordine at 02:34AM on November 02, 2014.
//!
//!

use std::num::NumCast;
use super::TimeSig;

pub type NumDiv = i64;

/// An enum with variants used to represent a musical division.
#[deriving(Show, Copy, Clone, FromPrimitive, RustcEncodable, RustcDecodable, PartialEq, Eq)]
pub enum Division {
    Bar,
    Minim,
    Beat,
    Quaver,
    SemiQuaver,
    ThirtySecond,
    SixtyFourth,
    OneHundredTwentyEighth,
    TwoHundredFiftySixth,
    FiveHundredTwelfth,
    OneThousandTwentyFourth,
    TotalDivisions
}

impl Division {

    pub fn from_int<T: ToPrimitive>(num: T) -> Division {
        ::std::num::from_int::<Division>(num.to_int().unwrap()).unwrap()
    }

    /// Convert to the equivalent duration as a number of Beats.
    pub fn beats(&self, ts: TimeSig) -> f64 {
        use std::num::Float;
        match *self {
            Division::Bar => ts.beats_in_a_bar(),
            _ => 2.0.powi(Division::Beat as i32 - *self as i32),
        }
    }

    /// Convert to the equivalent duration as a number of Bars.
    pub fn bars(&self, ts: TimeSig) -> f64 {
        match *self {
            Division::Bar => 1.0,
            _ => self.beats(ts) / ts.beats_in_a_bar(),
        }
    }

}

impl NumCast for Division {
    fn from<T: ToPrimitive>(n: T) -> Option<Division> {
        Some(Division::from_int(n.to_int().unwrap()))
    }
}

impl ToPrimitive for Division {
    fn to_i64(&self) -> Option<i64> {
        Some(match self {
            &Division::Bar => 0,
            &Division::Minim => 1,
            &Division::Beat => 2,
            &Division::Quaver => 3,
            &Division::SemiQuaver => 4,
            &Division::ThirtySecond => 5,
            &Division::SixtyFourth => 6,
            &Division::OneHundredTwentyEighth => 7,
            &Division::TwoHundredFiftySixth => 8,
            &Division::FiveHundredTwelfth => 9,
            &Division::OneThousandTwentyFourth => 10,
            &Division::TotalDivisions => 11,
        })
    }
    fn to_u64(&self) -> Option<u64> {
        Some(match self {
            &Division::Bar => 0,
            &Division::Minim => 1,
            &Division::Beat => 2,
            &Division::Quaver => 3,
            &Division::SemiQuaver => 4,
            &Division::ThirtySecond => 5,
            &Division::SixtyFourth => 6,
            &Division::OneHundredTwentyEighth => 7,
            &Division::TwoHundredFiftySixth => 8,
            &Division::FiveHundredTwelfth => 9,
            &Division::OneThousandTwentyFourth => 10,
            &Division::TotalDivisions => 11,
        })
    }
}

impl Add<int, int> for Division {
    fn add(self, rhs: int) -> int {
        self.to_int().unwrap() + rhs
    }
}

impl Sub<int, int> for Division {
    fn sub(self, rhs: int) -> int {
        self.to_int().unwrap() - rhs
    }
}

/// The 'Division Type'. Used for handling 'Thirds'.
/// Whole represents a Whole division, while TwoThirds
/// represents two thirds of a division.
#[deriving(Show, Copy, Clone, FromPrimitive, RustcEncodable, RustcDecodable, PartialEq, Eq)]
pub enum DivType {
    Whole, TwoThirds
}

impl DivType {
    pub fn from_int<T: NumCast>(num: T) -> DivType {
        ::std::num::from_int::<DivType>(num.to_int().unwrap()).unwrap()
    }
}

impl NumCast for DivType {
    fn from<T: ToPrimitive>(n: T) -> Option<DivType> {
        Some(DivType::from_int(n.to_int().unwrap()))
    }
}

impl ToPrimitive for DivType {
    fn to_i64(&self) -> Option<i64> {
        Some(match self {
            &DivType::Whole => 0i64,
            &DivType::TwoThirds => 1i64,
        })
    }
    fn to_u64(&self) -> Option<u64> {
        Some(match self {
            &DivType::Whole => 0u64,
            &DivType::TwoThirds => 1u64,
        })
    }
}

impl Add<int, int> for DivType {
    fn add(self, rhs: int) -> int {
        self.to_int().unwrap() + rhs
    }
}

impl Sub<int, int> for DivType {
    fn sub(self, rhs: int) -> int {
        self.to_int().unwrap() - rhs
    }
}

