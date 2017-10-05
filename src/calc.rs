//!
//!  time_calc.rs
//!
//!  Created by Mitchell Nordine at 05:04AM on November 01, 2014.
//!
//!

use super::{
    Measure,
    TimeSig,
};
use super::division::{
    NumDiv,
    Division,
    DivType,
};

pub type Bpm = f64;
pub type Ppqn = u32;
pub type Ms = f64;
pub type SampleHz = f64;
pub type Samples = i64;
pub type Ticks = i64;

pub const MINUTE_IN_MS: Ms = 60_000.0;
pub const SECOND_IN_MS: Ms =  1_000.0;

/// Calculate and return the duration of a bar in milliseconds.
#[inline]
pub fn bar_in_ms(bpm: Bpm, ts: TimeSig) -> Ms {
    beat_in_ms(bpm) * ts.beats_per_bar()
}

/// Calculate and return the duration of a beat in milliseconds.
#[inline]
pub fn beat_in_ms(bpm: Bpm) -> Ms {
    MINUTE_IN_MS / bpm
}

/// Calculate and return milliseconds from a given musical division.
#[inline]
pub fn ms_from_measure(num: NumDiv, div: Division, div_type: DivType, bpm: Bpm, ts: TimeSig) -> Ms {
    Measure(num, div, div_type).beats(ts) * beat_in_ms(bpm)
}

/// Calculate and return milliseconds from a given number of samples.
#[inline]
pub fn ms_from_samples(samples: Samples, sample_hz: SampleHz) -> Ms {
    samples as Ms * SECOND_IN_MS / sample_hz
}

/// Calculate and return milliseconds from a given number of ticks.
#[inline]
pub fn ms_from_ticks(ticks: Ticks, bpm: Bpm, ppqn: Ppqn) -> Ms {
    tick_in_ms(bpm, ppqn) * ticks as Ms
}

/// Calculate and return samples from a given musical division.
#[inline]
pub fn samples_from_measure(num: NumDiv, div: Division, div_type: DivType,
                            bpm: Bpm, ts: TimeSig, sample_hz: SampleHz) -> Samples {
    samples_from_ms(ms_from_measure(num, div, div_type, bpm, ts), sample_hz)
}

/// Calculate and return samples from a given number of milliseconds.
#[inline]
pub fn samples_from_ms(ms: Ms, sample_hz: SampleHz) -> Samples {
    (ms * sample_hz as Ms / SECOND_IN_MS) as Samples
}

/// Calculate and return samples from a given number of ticks.
#[inline]
pub fn samples_from_ticks(ticks: Ticks, bpm: Bpm, ppqn: Ppqn, sample_hz: SampleHz) -> Samples {
    samples_from_ms(ms_from_ticks(ticks, bpm, ppqn), sample_hz)
}

/// Calculate and return the duration of a tick in milliseconds.
#[inline]
pub fn tick_in_ms(bpm: Bpm, ppqn: Ppqn) -> Ms {
    beat_in_ms(bpm) / ppqn as Ms
}

/// Computes number of ticks from a musical measure considering time signature.
#[inline]
pub fn ticks_from_measure(num: NumDiv, div: Division, div_type: DivType,
                          ts: TimeSig, ppqn: Ppqn) -> Ticks {
    let num_quarters = match div_type {
        DivType::TwoThirds => div.beats(ts) * num as f64 * 2.0 / 3.0,
        DivType::Whole => div.beats(ts) * num as f64
    };
    (num_quarters * ppqn as f64).round() as Ticks
}

/// Calculate and return the number of ticks from a given number of milliseconds.
#[inline]
pub fn ticks_from_ms(ms: Ms, bpm: Bpm, ppqn: Ppqn) -> Ticks {
    (ms / tick_in_ms(bpm, ppqn)).round() as Ticks
}

/// Calculate and return a number of ticks from a given number of samples.
#[inline]
pub fn ticks_from_samples(samples: Samples, bpm: Bpm, ppqn: Ppqn, sample_hz: SampleHz) -> Ticks {
    ticks_from_ms(ms_from_samples(samples, sample_hz), bpm, ppqn)
}
