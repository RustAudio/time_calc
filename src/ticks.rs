//!
//!  ticks.rs
//!
//!  Created by Mitchell Nordine at 08:46PM on November 02, 2014.
//!
//!

use num::{FromPrimitive, ToPrimitive};
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use super::calc;
use super::{
    Bars,
    Bpm,
    Ms,
    ms_from_ticks,
    Ppqn,
    SampleHz,
    Samples,
    samples_from_ticks,
    TimeSig,
};

/// Time representation in the form of Ticks.
#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct Ticks(pub calc::Ticks);

impl Ticks {

    /// Return the unit value of Ticks.
    #[inline]
    pub fn ticks(&self) -> calc::Ticks { let Ticks(ticks) = *self; ticks }

    /// Convert to the equivalent duration as a number of Bars.
    #[inline]
    pub fn bars(&self, ts: TimeSig, ppqn: Ppqn) -> f64 {
        self.ticks() as f64 / Bars(1).ticks(ts, ppqn) as f64
    }
    /// Convert to the equivalent duration as a number of Beats.
    #[inline]
    pub fn beats(&self, ppqn: Ppqn) -> f64 {
        self.ticks() as f64 / ppqn as f64
    }

    /// Convert to the unit value of `Ms`.
    #[inline]
    pub fn ms(&self, bpm: Bpm, ppqn: Ppqn) -> calc::Ms {
        ms_from_ticks(self.ticks(), bpm, ppqn)
    }
    /// Convert to `Ms`.
    #[inline]
    pub fn to_ms(&self, bpm: Bpm, ppqn: Ppqn) -> Ms {
        Ms(self.ms(bpm, ppqn))
    }

    /// Convert to the unit value of `Samples`.
    #[inline]
    pub fn samples(&self, bpm: Bpm, ppqn: Ppqn, sample_hz: SampleHz) -> calc::Samples {
        samples_from_ticks(self.ticks(), bpm, ppqn, sample_hz)
    }

    /// Convert to `Samples`.
    #[inline]
    pub fn to_samples(&self, bpm: Bpm, ppqn: Ppqn, sample_hz: SampleHz) -> Samples {
        Samples(self.samples(bpm, ppqn, sample_hz))
    }

}

impl Add for Ticks {
    type Output = Ticks;
    #[inline]
    fn add(self, rhs: Ticks) -> Ticks {
        Ticks(self.ticks() + rhs.ticks())
    }
}

impl Sub for Ticks {
    type Output = Ticks;
    #[inline]
    fn sub(self, rhs: Ticks) -> Ticks {
        Ticks(self.ticks() - rhs.ticks())
    }
}

impl Mul for Ticks {
    type Output = Ticks;
    #[inline]
    fn mul(self, rhs: Ticks) -> Ticks {
        Ticks(self.ticks() * rhs.ticks())
    }
}

impl Div for Ticks {
    type Output = Ticks;
    #[inline]
    fn div(self, rhs: Ticks) -> Ticks {
        Ticks(self.ticks() / rhs.ticks())
    }
}

impl Rem for Ticks {
    type Output = Ticks;
    #[inline]
    fn rem(self, rhs: Ticks) -> Ticks {
        Ticks(self.ticks() % rhs.ticks())
    }
}

impl Neg for Ticks {
    type Output = Ticks;
    #[inline]
    fn neg(self) -> Ticks {
        Ticks(-self.ticks())
    }
}

impl PartialEq for Ticks {
    #[inline]
    fn eq(&self, other: &Ticks) -> bool {
        self.ticks() == other.ticks()
    }
}

impl Eq for Ticks {}

impl PartialOrd for Ticks {
    #[inline]
    fn partial_cmp(&self, other: &Ticks) -> Option<Ordering> {
        self.ticks().partial_cmp(&other.ticks())
    }
}

impl Ord for Ticks {
    #[inline]
    fn cmp(&self, other: &Ticks) -> Ordering {
        self.ticks().cmp(&other.ticks())
    }
}

impl ToPrimitive for Ticks {
    fn to_u64(&self) -> Option<u64> {
        self.ticks().to_u64()
    }
    fn to_i64(&self) -> Option<i64> {
        self.ticks().to_i64()
    }
}

impl FromPrimitive for Ticks {
    fn from_u64(n: u64) -> Option<Ticks> {
        Some(Ticks(n as calc::Ticks))
    }
    fn from_i64(n: i64) -> Option<Ticks> {
        Some(Ticks(n as calc::Ticks))
    }
}

