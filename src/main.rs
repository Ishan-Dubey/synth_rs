use cpal::traits::StreamTrait;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use synth_rs::{audio, input, synth::Synth};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Figure out sample rate & make the Synth
    let sample_rate = audio::default_sample_rate()?;
    let synth = Arc::new(Mutex::new(Synth::new(sample_rate)));

    // Wire up audio callback
    let stream = audio::build_output_stream(synth.clone())?;
    stream.play()?;

    // Spawn a keyboard thread
    let synth_kb = synth.clone();
    thread::spawn(move || {
        input::run_keyboard_loop(|freq, on| {
            let mut s = synth_kb.lock().unwrap();
            if on { s.note_on(freq) } else { s.note_off() }
        });
    });

    // Keep main alive forever
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
