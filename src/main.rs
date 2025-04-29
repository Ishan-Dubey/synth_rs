use std::{error::Error, sync::{Arc, Mutex}, thread, time::Duration};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossterm::event::{self, Event, KeyCode};
use std::f32::consts::PI;

struct Synth {
    sample_rate: f32,
    phase:       f32,
    freq:        f32,
    volume:      f32,
}

impl Synth {
    fn new(sample_rate: f32) -> Self {
        Synth { sample_rate, phase: 0.0, freq: 0.0, volume: 0.5 }
    }

    /// Generate the next sample (−1.0 to +1.0)
    fn next_sample(&mut self) -> f32 {
        let value = (2.0 * PI * self.phase).sin() * self.volume;
        // advance phase, wrap at 1.0
        self.phase = (self.phase + self.freq / self.sample_rate) % 1.0;
        value
    }

    fn set_freq(&mut self, f: f32) {
        self.freq = f;
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // 1) Set up host, device, and default output config
    let host   = cpal::default_host();
    let device = host.default_output_device().ok_or("No output device")?;
    let supported_config = device.default_output_config()?;
    let config: cpal::StreamConfig = supported_config.clone().into();
    let sample_rate = config.sample_rate.0 as f32;

    // 2) Shared synthesizer state
    let synth = Arc::new(Mutex::new(Synth::new(sample_rate)));
    let synth_cb = synth.clone();

    // 3) Build and run output stream
    let stream = device.build_output_stream::<f32, _, _>(
        &config.into(),
        move |data: &mut [f32], _| {
            let mut s = synth_cb.lock().unwrap();
            for sample in data {
                *sample = s.next_sample();
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;
    stream.play()?;

    // 4) Spawn keyboard‐listener thread
    let synth_input = synth.clone();
    thread::spawn(move || {
        // Map keys Z, X, C… to 16 semitones above A2=110 Hz :contentReference[oaicite:0]{index=0}
        let base = 110.0_f32;
        let twelfth_root = 2_f32.powf(1.0 / 12.0);
        let keys = ['z','x','c','v','b','n','m',',','.','/','s','d','g','h','j','l'];
        loop {
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let Event::Key(ev) = event::read().unwrap() {
                    let mut s = synth_input.lock().unwrap();
                    if let KeyCode::Char(ch) = ev.code {
                        if let Some(idx) = keys.iter().position(|&k| k == ch) {
                            let freq = base * twelfth_root.powf(idx as f32);
                            s.set_freq(freq);
                            println!("Note On: {:.2} Hz", freq);
                        } else {
                            // any other key = note off
                            s.set_freq(0.0);
                            println!("Note Off");
                        }
                    }
                }
            }
        }
    });

    // Keep main thread alive
    loop { thread::sleep(Duration::from_secs(1)); }
}
