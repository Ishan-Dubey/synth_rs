use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{OutputCallbackInfo, Stream, StreamConfig};

use crate::synth::Synth;

/// Query the default output device’s sample rate.
pub fn default_sample_rate() -> Result<f32, Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("No output device")?;
    let cfg = device.default_output_config()?;
    Ok(cfg.sample_rate().0 as f32)
}

/// Build a CPAL output stream that calls `s.next_sample()`.
pub fn build_output_stream(synth: Arc<Mutex<Synth>>) -> Result<Stream, Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("No output device")?;
    let cfg = device.default_output_config()?;
    let config: StreamConfig = cfg.into();

    let synth_cb = synth.clone();
    let stream = device.build_output_stream::<f32, _, _>(
        &config,
        move |data: &mut [f32], _: &OutputCallbackInfo| {
            let mut s = synth_cb.lock().unwrap();
            for sample in data.iter_mut() {
                *sample = s.next_sample();
            }
        },
        move |err| eprintln!("Stream error: {}", err),
        None,
    )?;
    Ok(stream)
}
