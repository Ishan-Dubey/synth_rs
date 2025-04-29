use std::f32::consts::PI;

/// A single‐voice sine‐wave synth.
pub struct Synth {
    sample_rate: f32,
    phase:       f32,
    freq:        f32,
    volume:      f32,
}

impl Synth {
    pub fn new(sample_rate: f32) -> Self {
        Synth {
            sample_rate,
            phase: 0.0,
            freq: 0.0,
            volume: 0.5,
        }
    }

    /// Called every audio frame to produce one sample (–1.0 to +1.0).
    pub fn next_sample(&mut self) -> f32 {
        let v = (2.0 * PI * self.phase).sin() * self.volume;
        self.phase = (self.phase + self.freq / self.sample_rate) % 1.0;
        v
    }

    /// Turn a note on by setting frequency.
    pub fn note_on(&mut self, freq: f32) {
        self.freq = freq;
    }

    /// Note off: silence the voice.
    pub fn note_off(&mut self) {
        self.freq = 0.0;
    }
}
