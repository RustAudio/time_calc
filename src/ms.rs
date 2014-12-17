//!
//!  ms.rs
//!
//!  Created by Mitchell Nordine at 08:22PM on November 02, 2014.
//!
//!

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
#[deriving(Show, Copy, Clone, Encodable, Decodable)]
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

impl Add<Ms, Ms> for Ms {
    #[inline]
    fn add(self, rhs: Ms) -> Ms {
        Ms(self.ms() + rhs.ms())
    }
}

impl Sub<Ms, Ms> for Ms {
    #[inline]
    fn sub(self, rhs: Ms) -> Ms {
        Ms(self.ms() - rhs.ms())
    }
}

impl Mul<Ms, Ms> for Ms {
    #[inline]
    fn mul(self, rhs: Ms) -> Ms {
        Ms(self.ms() * rhs.ms())
    }
}

impl Div<Ms, Ms> for Ms {
    #[inline]
    fn div(self, rhs: Ms) -> Ms {
        Ms(self.ms() / rhs.ms())
    }
}

impl Rem<Ms, Ms> for Ms {
    #[inline]
    fn rem(self, rhs: Ms) -> Ms {
        Ms(self.ms() % rhs.ms())
    }
}

impl Neg<Ms> for Ms {
    #[inline]
    fn neg(&self) -> Ms {
        Ms(-self.ms())
    }
}

impl PartialEq for Ms {
    #[inline]
    fn eq(&self, other: &Ms) -> bool {
        self.ms() == other.ms()
    }
}

impl Eq for Ms {}

impl PartialOrd for Ms {
    #[inline]
    fn partial_cmp(&self, other: &Ms) -> Option<Ordering> {
        self.ms().partial_cmp(&other.ms())
    }
}

impl Ord for Ms {
    #[inline]
    fn cmp(&self, other: &Ms) -> Ordering {
        self.partial_cmp(other).unwrap()
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

