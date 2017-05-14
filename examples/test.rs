//! 
//! An example of the time_calc crate in action.
//!

extern crate time_calc;

use time_calc::{
    Bars,
    Beats,
    Bpm,
    Division,
    DivType,
    Measure,
    Ms,
    Ppqn,
    SampleHz,
    Samples,
    Ticks,
    TimeSig,
};

// "Samples per second" is used to convert between samples and milliseconds.
const SAMPLE_HZ: SampleHz = 44_100.0;
// "Parts per quarter note" is used to calculate Ticks; a high resolution musical time measurement.
const PPQN: Ppqn = 19_200;

fn main() {
    println!("time_calc demonstration!");

    // Out `Bars` type is a simplified version of a Measure.
    assert!(Bars(1).measure() == Measure(1, Division::Bar, DivType::Whole));
    // The same goes for out `Beats` type.
    assert!(Beats(1).measure() == Measure(1, Division::Beat, DivType::Whole));

    // We can use "parts per quarter note" to convert to ticks.
    println!("Parts per quarter note: {}", PPQN);
    println!("Duration of a beat in ticks: {}", Beats(1).ticks(PPQN));
    println!("Duration of 38_400 ticks in beats: {}", Ticks(38_400).beats(PPQN));

    // We can use "beats per minute" to convert from musical time to milliseconds.
    let bpm: Bpm = 120.0;
    println!("Duration of a beat at 120 beats per minute: {} milliseconds.", Beats(1).ms(bpm));

    // And we can use "samples per second" to convert our duration to samples.
    println!("Samples per second: {}", SAMPLE_HZ);
    println!("Duration of a beat at 120bpm in samples: {}", Beats(1).samples(bpm, SAMPLE_HZ));

    // We also need to know the "time signature" if we are to convert from "Bars".
    // This is because different time signatures can have a different duration in "Beats".
    let beats_per_bar = TimeSig { top: 4, bottom: 4 }.beats_per_bar();
    println!("Duration of a bar in `Beats` with a 4/4 Time Signature: {}", beats_per_bar);
    let beats_per_bar = TimeSig { top: 3, bottom: 4 }.beats_per_bar();
    println!("Duration of a bar in `Beats` with a 3/4 Time Signature: {}", beats_per_bar);
    let beats_per_bar = TimeSig { top: 7, bottom: 8 }.beats_per_bar();
    println!("Duration of a bar in `Beats` with a 7/8 Time Signature: {}", beats_per_bar);
    let time_sig = TimeSig { top: 4, bottom: 4 };
    println!("Duration of a bar at 120bpm, 44_100 sample_hz and 4/4 Time Sig in samples: {}",
             Bars(1).samples(bpm, time_sig, SAMPLE_HZ));

    // We can also convert back the other way! Here's an example from Ms -> Beats.
    println!("1 minute as a duration in beats: {}", Ms(60_000.0).beats(bpm));

    // Here's an example from Samples -> Bars.
    println!("176_400 samples as a duration in bars: {}",
             Samples(176_400).bars(bpm, time_sig, SAMPLE_HZ));

    println!("Great Success!");
}
