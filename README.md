time_calc
=========

A library for music/DSP time conversions!

`time_calc` provides functions and methods for converting between ticks, ms, samples, bars, beats and measures.

It looks like this:

```Rust
const SAMPLE_HZ: SampleHz = 44_100.0;
let bpm: Bpm = 120.0;
let time_sig = TimeSig { top: 4, bottom: 4 };
println!("Convert 4 bars to samples where the tempo is 120bpm, the time signature is 4/4
         and the sample rate is 44,100 samples per second: {}",
         Bars(4).samples(bpm, time_sig, SAMPLE_HZ))
```

See [the example](https://github.com/RustAudio/time_calc/blob/master/examples/test.rs) for a better demo.
