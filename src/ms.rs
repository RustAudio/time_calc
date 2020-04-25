//!
//!  ms.rs
//!
//!  Created by Mitchell Nordine at 08:22PM on November 02, 2014.
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
    Ppqn,
    SampleHz,
    Samples,
    Ticks,
    TimeSig,
    samples_from_ms,
    ticks_from_ms,
};

/// Time representation in the form of Milliseconds.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ms(pub calc::Ms);

impl Ms {

    /// Return the unit value of Ms.
    #[inline]
    pub fn ms(&self) -> calc::Ms { let Ms(ms) = *self; ms }

    /// Convert to the equivalent duration in Bars.
    #[inline]
    pub fn bars(&self, bpm: Bpm, ts: TimeSig) -> f64 {
        self.ms() as f64 / Bars(1).ms(bpm, ts) as f64
    }

    /// Convert to the equivalent duration in Beats.
    #[inline]
    pub fn beats(&self, bpm: Bpm) -> f64 {
        self.ms() as f64 / Beats(1).ms(bpm) as f64
    }

    /// Convert to unit value of `Samples`.
    #[inline]
    pub fn samples(&self, sample_hz: SampleHz) -> calc::Samples {
        samples_from_ms(self.ms(), sample_hz)
    }
    /// Convert to `Samples`.
    #[inline]
    pub fn to_samples(&self, sample_hz: SampleHz) -> Samples {
        Samples(self.samples(sample_hz))
    }

    /// Convert to unit value of `Ticks`.
    #[inline]
    pub fn ticks(&self, bpm: Bpm, ppqn: Ppqn) -> calc::Ticks {
        ticks_from_ms(self.ms(), bpm, ppqn)
    }
    /// Convert to `Ticks`.
    #[inline]
    pub fn to_ticks(&self, bpm: Bpm, ppqn: Ppqn) -> Ticks {
        Ticks(self.ticks(bpm, ppqn))
    }

}

impl From<calc::Ms> for Ms {
    #[inline]
    fn from(ms: calc::Ms) -> Self {
        Ms(ms)
    }
}

impl Add for Ms {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.ms() + rhs.ms())
    }
}

impl Sub for Ms {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.ms() - rhs.ms())
    }
}

impl Mul for Ms {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.ms() * rhs.ms())
    }
}

impl Div for Ms {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(self.ms() / rhs.ms())
    }
}

impl Rem for Ms {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self(self.ms() % rhs.ms())
    }
}

impl Neg for Ms {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.ms())
    }
}

impl AddAssign for Ms {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Ms {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Ms {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Ms {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl RemAssign for Ms {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl ToPrimitive for Ms {
    fn to_u64(&self) -> Option<u64> {
        self.ms().to_u64()
    }
    fn to_i64(&self) -> Option<i64> {
        self.ms().to_i64()
    }
}

impl FromPrimitive for Ms {
    fn from_u64(n: u64) -> Option<Ms> {
        Some(Ms(n as calc::Ms))
    }
    fn from_i64(n: i64) -> Option<Ms> {
        Some(Ms(n as calc::Ms))
    }
}
