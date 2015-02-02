//!
//!  samples.rs
//!
//!  Created by Mitchell Nordine at 08:40PM on November 02, 2014.
//!
//!

use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::num::{FromPrimitive, ToPrimitive};
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
#[derive(Debug, Copy, Clone, RustcEncodable, RustcDecodable)]
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

impl Add for Samples {
    type Output = Samples;
    #[inline]
    fn add(self, rhs: Samples) -> Samples {
        Samples(self.samples() + rhs.samples())
    }
}

impl Sub for Samples {
    type Output = Samples;
    #[inline]
    fn sub(self, rhs: Samples) -> Samples {
        Samples(self.samples() - rhs.samples())
    }
}

impl Mul for Samples {
    type Output = Samples;
    #[inline]
    fn mul(self, rhs: Samples) -> Samples {
        Samples(self.samples() * rhs.samples())
    }
}

impl Div for Samples {
    type Output = Samples;
    #[inline]
    fn div(self, rhs: Samples) -> Samples {
        Samples(self.samples() / rhs.samples())
    }
}

impl Rem for Samples {
    type Output = Samples;
    #[inline]
    fn rem(self, rhs: Samples) -> Samples {
        Samples(self.samples() % rhs.samples())
    }
}

impl Neg for Samples {
    type Output = Samples;
    #[inline]
    fn neg(self) -> Samples {
        Samples(-self.samples())
    }
}

impl PartialEq for Samples {
    #[inline]
    fn eq(&self, other: &Samples) -> bool {
        self.samples() == other.samples()
    }
}

impl Eq for Samples {}

impl PartialOrd for Samples {
    #[inline]
    fn partial_cmp(&self, other: &Samples) -> Option<Ordering> {
        self.samples().partial_cmp(&other.samples())
    }
}

impl Ord for Samples {
    #[inline]
    fn cmp(&self, other: &Samples) -> Ordering {
        self.samples().cmp(&other.samples())
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

