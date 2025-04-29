// src/oscillator.rs

use std::f32::consts::PI;

/// A generic oscillator: advance its phase and get the next sample.
pub trait Oscillator: Send {
    /// Advance internal state and return the next sample in [−1.0, +1.0].
    fn next_sample(&mut self) -> f32;
    /// Set the oscillator’s frequency in Hz.
    fn set_freq(&mut self, freq: f32);
}

/// A simple sine-wave oscillator.
pub struct SineOsc {
    phase: f32,
    freq: f32,
    sample_rate: f32,
}

impl SineOsc {
    /// Create a new sine oscillator at 0 Hz.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            freq: 0.0,
            sample_rate,
        }
    }
}

impl Oscillator for SineOsc {
    fn next_sample(&mut self) -> f32 {
        let out = (2.0 * PI * self.phase).sin();
        // advance phase, wrap at 1.0
        self.phase = (self.phase + self.freq / self.sample_rate) % 1.0;
        out
    }

    fn set_freq(&mut self, freq: f32) {
        self.freq = freq;
    }
}
