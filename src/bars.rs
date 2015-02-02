//!
//!  bars.rs
//!
//!  Created by Mitchell Nordine at 04:12AM on December 06, 2014.
//!
//!

use std::cmp::Ordering;
use std::num::{FromPrimitive, ToPrimitive};
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use super::calc;
use super::{
    Bpm,
    Division,
    DivType,
    Measure,
    Ms,
    NumDiv,
    Ppqn,
    SampleHz,
    Samples,
    Ticks,
    TimeSig,
    ms_from_measure,
    samples_from_measure,
    ticks_from_measure,
};

/// Represents a number of bars aka a simplified version of `Measure(1, Bar, Whole)`.
#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
pub struct Bars(pub NumDiv);

impl Bars {

    /// Return the number of bars.
    #[inline]
    pub fn bars(&self) -> NumDiv { let Bars(num) = *self; num }

    /// Convert to the equivalent duration as a number of Beats.
    #[inline]
    pub fn beats(&self, ts: TimeSig) -> f64 { Division::Bar.beats(ts) }

    /// Convert to a `Measure`.
    #[inline]
    pub fn measure(&self) -> Measure { Measure(self.bars(), Division::Bar, DivType::Whole) }

    /// Convert to the unit value of `Ms`.
    #[inline]
    pub fn ms(&self, bpm: Bpm, ts: TimeSig) -> calc::Ms {
        ms_from_measure(self.bars(), Division::Bar, DivType::Whole, bpm, ts)
    }
    /// Convert to `Ms`.
    #[inline]
    pub fn to_ms(&self, bpm: Bpm, ts: TimeSig) -> Ms {
        Ms(self.ms(bpm, ts))
    }

    /// Convert to the unit value of `Samples`.
    #[inline]
    pub fn samples(&self, bpm: Bpm, ts: TimeSig, sample_hz: SampleHz) -> calc::Samples {
        samples_from_measure(self.bars(), Division::Bar, DivType::Whole, bpm, ts, sample_hz)
    }
    /// Convert to `Samples`.
    #[inline]
    pub fn to_samples(&self, bpm: Bpm, ts: TimeSig, sample_hz: SampleHz) -> Samples {
        Samples(self.samples(bpm, ts, sample_hz))
    }

    /// Convert to the unit value of `Ticks`.
    #[inline]
    pub fn ticks(&self, ts: TimeSig, ppqn: Ppqn) -> calc::Ticks {
        ticks_from_measure(self.bars(), Division::Bar, DivType::Whole, ts, ppqn)
    }
    /// Convert to `Ticks`.
    #[inline]
    pub fn to_ticks(&self, ts: TimeSig, ppqn: Ppqn) -> Ticks {
        Ticks(self.ticks(ts, ppqn))
    }

}

impl Add for Bars {
    type Output = Bars;
    #[inline]
    fn add(self, rhs: Bars) -> Bars {
        Bars(self.bars() + rhs.bars())
    }
}

impl Sub for Bars {
    type Output = Bars;
    #[inline]
    fn sub(self, rhs: Bars) -> Bars {
        Bars(self.bars() - rhs.bars())
    }
}

impl Mul for Bars {
    type Output = Bars;
    #[inline]
    fn mul(self, rhs: Bars) -> Bars {
        Bars(self.bars() * rhs.bars())
    }
}

impl Div for Bars {
    type Output = Bars;
    #[inline]
    fn div(self, rhs: Bars) -> Bars {
        Bars(self.bars() / rhs.bars())
    }
}

impl Rem for Bars {
    type Output = Bars;
    #[inline]
    fn rem(self, rhs: Bars) -> Bars {
        Bars(self.bars() % rhs.bars())
    }
}

impl Neg for Bars {
    type Output = Bars;
    #[inline]
    fn neg(self) -> Bars {
        Bars(-self.bars())
    }
}

impl PartialEq for Bars {
    #[inline]
    fn eq(&self, other: &Bars) -> bool {
        self.bars() == other.bars()
    }
}

impl Eq for Bars {}

impl PartialOrd for Bars {
    #[inline]
    fn partial_cmp(&self, other: &Bars) -> Option<Ordering> {
        self.bars().partial_cmp(&other.bars())
    }
}

impl Ord for Bars {
    #[inline]
    fn cmp(&self, other: &Bars) -> Ordering {
        self.bars().cmp(&other.bars())
    }
}

impl ToPrimitive for Bars {
    fn to_u64(&self) -> Option<u64> {
        self.bars().to_u64()
    }
    fn to_i64(&self) -> Option<i64> {
        self.bars().to_i64()
    }
}

impl FromPrimitive for Bars {
    fn from_u64(n: u64) -> Option<Bars> {
        Some(Bars(n as NumDiv))
    }
    fn from_i64(n: i64) -> Option<Bars> {
        Some(Bars(n as NumDiv))
    }
}

