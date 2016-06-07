//!
//!  division.rs
//!
//!  Created by Mitchell Nordine at 02:34AM on November 02, 2014.
//!
//!

use num::{NumCast, FromPrimitive, ToPrimitive};
use std::ops::{Add, Sub};
use super::TimeSig;

pub type NumDiv = i64;

/// An enum with variants used to represent a musical division.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
}

const HIGHEST_ZOOM_STEP: u8 = 10;



impl Division {

    pub fn from_isize<T: ToPrimitive>(num: T) -> Division {
        FromPrimitive::from_isize(num.to_isize().unwrap()).unwrap()
    }

    /// Convert to the equivalent duration as a number of Beats.
    pub fn beats(&self, ts: TimeSig) -> f64 {
        use num::Float;
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

    /// Zoom into a higher resolution division by the number of steps given.
    pub fn zoom_in(&self, steps: u8) -> Option<Division> {
        let zoom_step = self.to_u8() + steps;
        if zoom_step <= HIGHEST_ZOOM_STEP {
            Some(NumCast::from(zoom_step).unwrap())
        } else {
            None
        }
    }

    /// Zoom into a higher resolution division by the number of steps given.
    pub fn zoom_out(&self, steps: u8) -> Option<Division> {
        let current_zoom_step = self.to_u8();
        if steps <= current_zoom_step {
            let zoom_step = current_zoom_step - steps;
            Some(NumCast::from(zoom_step).unwrap())
        } else {
            None
        }
    }

    /// Convert a Division to its byte equivalent.
    pub fn to_u8(&self) -> u8 {
        ToPrimitive::to_u8(self).unwrap()
    }

    /// Convert a Division to its signed byte equivalent.
    pub fn to_i8(&self) -> i8 {
        ToPrimitive::to_i8(self).unwrap()
    }

}

impl NumCast for Division {
    fn from<T: ToPrimitive>(n: T) -> Option<Division> {
        Some(Division::from_isize(n.to_isize().unwrap()))
    }
}

impl FromPrimitive for Division {
    fn from_i64(n: i64) -> Option<Self> { Self::from_u64(n as u64) }
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0  => Some(Division::Bar),
            1  => Some(Division::Minim),
            2  => Some(Division::Beat),
            3  => Some(Division::Quaver),
            4  => Some(Division::SemiQuaver),
            5  => Some(Division::ThirtySecond),
            6  => Some(Division::SixtyFourth),
            7  => Some(Division::OneHundredTwentyEighth),
            8  => Some(Division::TwoHundredFiftySixth),
            9  => Some(Division::FiveHundredTwelfth),
            10 => Some(Division::OneThousandTwentyFourth),
            _  => None,
        }
    }
}

impl ToPrimitive for Division {
    fn to_i64(&self) -> Option<i64> { self.to_u64().map(|n| n as i64) }
    fn to_u64(&self) -> Option<u64> {
        Some(match *self {
            Division::Bar => 0,
            Division::Minim => 1,
            Division::Beat => 2,
            Division::Quaver => 3,
            Division::SemiQuaver => 4,
            Division::ThirtySecond => 5,
            Division::SixtyFourth => 6,
            Division::OneHundredTwentyEighth => 7,
            Division::TwoHundredFiftySixth => 8,
            Division::FiveHundredTwelfth => 9,
            Division::OneThousandTwentyFourth => 10,
        })
    }
}

impl Add<isize> for Division {
    type Output = isize;
    fn add(self, rhs: isize) -> isize {
        self.to_isize().unwrap() + rhs
    }
}

impl Sub<isize> for Division {
    type Output = isize;
    fn sub(self, rhs: isize) -> isize {
        self.to_isize().unwrap() - rhs
    }
}

/// The 'Division Type'. Used for handling 'Thirds'.
/// Whole represents a Whole division, while TwoThirds represents two thirds of a division.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DivType {
    Whole,
    TwoThirds,
}


impl DivType {

    pub fn from_isize<T: NumCast>(num: T) -> DivType {
        FromPrimitive::from_isize(num.to_isize().unwrap()).unwrap()
    }

}

impl NumCast for DivType {
    fn from<T: ToPrimitive>(n: T) -> Option<DivType> {
        Some(DivType::from_isize(n.to_isize().unwrap()))
    }
}

impl ::rand::Rand for DivType {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> DivType {
        let rand: bool = rng.gen();
        if rand { DivType::Whole } else { DivType::TwoThirds }
    }
}

impl FromPrimitive for DivType {
    fn from_i64(n: i64) -> Option<Self> { Self::from_u64(n as u64) }
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(DivType::Whole),
            1 => Some(DivType::TwoThirds),
            _ => None,
        }
    }
}

impl ToPrimitive for DivType {
    fn to_i64(&self) -> Option<i64> { self.to_u64().map(|n| n as i64) }
    fn to_u64(&self) -> Option<u64> {
        Some(match self {
            &DivType::Whole     => 0,
            &DivType::TwoThirds => 1,
        })
    }
}

impl Add<isize> for DivType {
    type Output = isize;
    fn add(self, rhs: isize) -> isize {
        self.to_isize().unwrap() + rhs
    }
}

impl Sub<isize> for DivType {
    type Output = isize;
    fn sub(self, rhs: isize) -> isize {
        self.to_isize().unwrap() - rhs
    }
}

