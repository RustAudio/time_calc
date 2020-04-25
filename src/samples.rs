//!
//!  samples.rs
//!
//!  Created by Mitchell Nordine at 08:40PM on November 02, 2014.
//!
//!

use num::{FromPrimitive, ToPrimitive};
use std::ops::{Add, Sub, Mul, Div, Rem, Neg,
               AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use super::calc;
use super::{
    Bars,
    Beats,
    Bpm,
    Ms,
    Ppqn,
    SampleHz,
    Ticks,
    ms_from_samples,
    ticks_from_samples,
    TimeSig,
};

/// Time representation in the form of Samples.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Samples(pub calc::Samples);

impl Samples {

    /// Return Samples as it's unit value.
    #[inline]
    pub fn samples(&self) -> calc::Samples { let Samples(samples) = *self; samples }

    /// Convert to the equivalent duration in Bars.
    #[inline]
    pub fn bars(&self, bpm: Bpm, ts: TimeSig, sample_hz: SampleHz) -> f64 {
        self.samples() as f64 / Bars(1).samples(bpm, ts, sample_hz) as f64
    }

    /// Convert to the equivalent duration in Beats.
    #[inline]
    pub fn beats(&self, bpm: Bpm, sample_hz: SampleHz) -> f64 {
        self.samples() as f64 / Beats(1).samples(bpm, sample_hz) as f64
    }

    /// Convert to the unit value of `Ms`.
    #[inline]
    pub fn ms(&self, sample_hz: SampleHz) -> calc::Ms {
        ms_from_samples(self.samples(), sample_hz)
    }
    /// Convert to `Ms`.
    #[inline]
    pub fn to_ms(&self, sample_hz: SampleHz) -> Ms {
        Ms(self.ms(sample_hz))
    }

    /// Convert to the unit value of `Ticks`.
    #[inline]
    pub fn ticks(&self, bpm: Bpm, ppqn: Ppqn, sample_hz: SampleHz) -> calc::Ticks {
        ticks_from_samples(self.samples(), bpm, ppqn, sample_hz)
    }
    /// Convert to `Ticks`.
    #[inline]
    pub fn to_ticks(&self, bpm: Bpm, ppqn: Ppqn, sample_hz: SampleHz) -> Ticks {
        Ticks(self.ticks(bpm, ppqn, sample_hz))
    }

}

impl From<calc::Samples> for Samples {
    fn from(samples: calc::Samples) -> Self {
        Self(samples)
    }
}

impl Add for Samples {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.samples() + rhs.samples())
    }
}

impl Sub for Samples {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.samples() - rhs.samples())
    }
}

impl Mul for Samples {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.samples() * rhs.samples())
    }
}

impl Div for Samples {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(self.samples() / rhs.samples())
    }
}

impl Rem for Samples {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self(self.samples() % rhs.samples())
    }
}

impl Neg for Samples {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.samples())
    }
}

impl AddAssign for Samples {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Samples {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Samples {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Samples {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl RemAssign for Samples {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl ToPrimitive for Samples {
    fn to_u64(&self) -> Option<u64> {
        self.samples().to_u64()
    }
    fn to_i64(&self) -> Option<i64> {
        self.samples().to_i64()
    }
}

impl FromPrimitive for Samples {
    fn from_u64(n: u64) -> Option<Samples> {
        Some(Samples(n as calc::Samples))
    }
    fn from_i64(n: i64) -> Option<Samples> {
        Some(Samples(n as calc::Samples))
    }
}
