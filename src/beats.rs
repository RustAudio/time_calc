//!
//!  beats.rs
//!
//!  Created by Mitchell Nordine at 06:25PM on December 06, 2014.
//!
//!

use num::{FromPrimitive, ToPrimitive};
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
    beat_in_ms,
    samples_from_ms,
};

/// Represents a number of beats aka a simplified version of `Measure(1, Beat, Whole)`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Beats(pub NumDiv);

impl Beats {

    /// Return the number of beats.
    #[inline]
    pub fn beats(&self) -> NumDiv { let Beats(num) = *self; num }

    /// Convert to the equivalent duration as a number of Bars.
    #[inline]
    pub fn bars(&self, ts: TimeSig) -> f64 { Division::Beat.bars(ts) }

    /// Convert to a `Measure`.
    #[inline]
    pub fn measure(&self) -> Measure { Measure(self.beats(), Division::Beat, DivType::Whole) }

    /// Convert to the unit value of `Ms`.
    #[inline]
    pub fn ms(&self, bpm: Bpm) -> calc::Ms {
        self.beats() as calc::Ms * beat_in_ms(bpm)
    }
    /// Convert to `Ms`.
    #[inline]
    pub fn to_ms(&self, bpm: Bpm) -> Ms {
        Ms(self.ms(bpm))
    }

    /// Convert to the unit value of `Samples`.
    #[inline]
    pub fn samples(&self, bpm: Bpm, sample_hz: SampleHz) -> calc::Samples {
        samples_from_ms(self.ms(bpm), sample_hz)
    }
    /// Conver to `Samples.
    #[inline]
    pub fn to_samples(&self, bpm: Bpm, sample_hz: SampleHz) -> Samples {
        Samples(self.samples(bpm, sample_hz))
    }

    /// Convert to the unit value of `Ticks`.
    #[inline]
    pub fn ticks(&self, ppqn: Ppqn) -> calc::Ticks {
        self.beats() as calc::Ticks * ppqn as calc::Ticks
    }

    /// Convert to `Ticks`.
    #[inline]
    pub fn to_ticks(&self, ppqn: Ppqn) -> Ticks {
        Ticks(self.ticks(ppqn))
    }

}

impl From<NumDiv> for Beats {
    fn from(n: NumDiv) -> Self {
        Beats(n)
    }
}

impl Add for Beats {
    type Output = Beats;
    #[inline]
    fn add(self, rhs: Beats) -> Beats {
        Beats(self.beats() + rhs.beats())
    }
}

impl Sub for Beats {
    type Output = Beats;
    #[inline]
    fn sub(self, rhs: Beats) -> Beats {
        Beats(self.beats() - rhs.beats())
    }
}

impl Mul for Beats {
    type Output = Beats;
    #[inline]
    fn mul(self, rhs: Beats) -> Beats {
        Beats(self.beats() * rhs.beats())
    }
}

impl Div for Beats {
    type Output = Beats;
    #[inline]
    fn div(self, rhs: Beats) -> Beats {
        Beats(self.beats() / rhs.beats())
    }
}

impl Rem for Beats {
    type Output = Beats;
    #[inline]
    fn rem(self, rhs: Beats) -> Beats {
        Beats(self.beats() % rhs.beats())
    }
}

impl Neg for Beats {
    type Output = Beats;
    #[inline]
    fn neg(self) -> Beats {
        Beats(-self.beats())
    }
}

impl ToPrimitive for Beats {
    fn to_u64(&self) -> Option<u64> {
        self.beats().to_u64()
    }
    fn to_i64(&self) -> Option<i64> {
        self.beats().to_i64()
    }
}

impl FromPrimitive for Beats {
    fn from_u64(n: u64) -> Option<Beats> {
        Some(Beats(n as NumDiv))
    }
    fn from_i64(n: i64) -> Option<Beats> {
        Some(Beats(n as NumDiv))
    }
}

