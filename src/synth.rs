use crate::oscillator::{Oscillator, SineOsc};

/// A single‐voice sine‐wave synth.
pub struct Synth {
    sample_rate: f32,
    osc: SineOsc,
    volume: f32,
}

impl Synth {
    pub fn new(sample_rate: f32) -> Self {
        Synth {
            sample_rate,
            osc: SineOsc::new(sample_rate),
            volume: 1.0,
        }
    }

    /// Call every audio frame to produce one sample (–1.0 to +1.0).
    pub fn next_sample(&mut self) -> f32 {
        self.osc.next_sample() * self.volume
    }

    /// Turn a note on by setting frequency.
    pub fn note_on(&mut self, freq: f32) {
        self.osc.set_freq(freq);
    }

    /// Note off: silence the voice.
    pub fn note_off(&mut self) {
        self.osc.set_freq(0.0);
    }
}
