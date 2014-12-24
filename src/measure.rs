//!
//!  measure.rs
//!
//!  Created by Mitchell Nordine at 08:33PM on November 02, 2014.
//!
//!

use super::calc;
use super::{
    Bpm,
    Division,
    DivType,
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

/// Time representation in the form of a Musical Measure.
///
/// i.e.
/// Measure(1, Bar, Whole) is one bar of musical time.
/// Measure(3, Beat, Whole) is three beats of musical time.
/// Measure(1, Minim, TwoThirds) is two thirds of a minim.
#[deriving(Show, Copy, Clone, RustcEncodable, RustcDecodable, PartialEq, Eq)]
pub struct Measure(pub NumDiv, pub Division, pub DivType);

impl Measure {

    /// Return the number of divisions.
    #[inline]
    pub fn num(&self) -> NumDiv {
        let Measure(num, _, _) = *self;
        num
    }
    /// Return the division measure.
    #[inline]
    pub fn div(&self) -> Division {
        let Measure(_, div, _) = *self;
        div
    }
    /// Return the division type.
    #[inline]
    pub fn div_type(&self) -> DivType {
        let Measure(_, _, div_type) = *self;
        div_type
    }

    /// Convert to the equivalent duration in Beats.
    #[inline]
    pub fn beats(&self, ts: TimeSig) -> f64 {
        let Measure(num, div, div_type) = *self;
        match div_type {
            DivType::Whole => div.beats(ts) * num as f64,
            DivType::TwoThirds => div.beats(ts) * num as f64 * (2.0 / 3.0),
        }
    }
    /// Convert to the equivalent duration in Bars.
    #[inline]
    pub fn bars(&self, ts: TimeSig) -> f64 {
        self.beats(ts) / Measure(1, Division::Bar, DivType::Whole).beats(ts)
    }

    /// Convert to the unit value of `Ms`.
    #[inline]
    pub fn ms(&self, bpm: Bpm, ts: TimeSig) -> calc::Ms {
        let Measure(num, div, div_type) = *self;
        ms_from_measure(num, div, div_type, bpm, ts)
    }
    /// Convert to `Ms`.
    #[inline]
    pub fn to_ms(&self, bpm: Bpm, ts: TimeSig) -> Ms {
        Ms(self.ms(bpm, ts))
    }

    /// Convert to the unit value of `Samples`.
    #[inline]
    pub fn samples(&self, bpm: Bpm, ts: TimeSig, sample_hz: SampleHz) -> calc::Samples {
        let Measure(num, div, div_type) = *self;
        samples_from_measure(num, div, div_type, bpm, ts, sample_hz)
    }
    /// Convert to `Samples`.
    #[inline]
    pub fn to_samples(&self, bpm: Bpm, ts: TimeSig, sample_hz: SampleHz) -> Samples {
        Samples(self.samples(bpm, ts, sample_hz))
    }

    /// Convert to the unit value of `Ticks`.
    #[inline]
    pub fn ticks(&self, ts: TimeSig, ppqn: Ppqn) -> calc::Ticks {
        let Measure(num, div, div_type) = *self;
        ticks_from_measure(num, div, div_type, ts, ppqn)
    }
    /// Convert to `Ticks`.
    #[inline]
    pub fn to_ticks(&self, ts: TimeSig, ppqn: Ppqn) -> Ticks {
        Ticks(self.ticks(ts, ppqn))
    }

}

