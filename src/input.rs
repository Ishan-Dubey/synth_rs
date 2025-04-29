use crossterm::{
    event::{self, Event, KeyCode},
    terminal::enable_raw_mode,
};
use std::time::Duration;

/// Poll for keys every 50 ms and call `callback(freq, on)`
/// on key presses/releases.
pub fn run_keyboard_loop<F>(mut callback: F)
where
    F: FnMut(f32, bool),
{
    // ensure raw mode is on
    enable_raw_mode().unwrap();

    let base = 110.0_f32;                 // A2 = 110 Hz
    let step = 2.0_f32.powf(1.0 / 12.0);  // semitone ratio
    // let keys = [
    //     'z','x','c','v','b','n','m',',','.','/',
    //     's','d','g','h','j','l',
    // ];
    let keys = [
        'z', 's', 'x', 'c', 'f', 'v', 'g', 'b', 'n', 'j', 'm', 'k', ',', 'l', '.', '/', '\''
    ];

    loop {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(ev) = event::read().unwrap() {
                if let KeyCode::Char(ch) = ev.code {
                    if let Some(idx) = keys.iter().position(|&k| k == ch) {
                        let freq = base * step.powf(idx as f32);
                        callback(freq, true);
                        continue;
                    }
                }
                // any other key = note off
                callback(0.0, false);
            }
        }
    }
}
