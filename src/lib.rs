
pub mod voice;
pub mod sound;
pub mod time;

pub use chrono;

use std::time::Instant;

pub fn sleep_until(next: Instant) {
    let now = Instant::now();
    if next > now {
        std::thread::sleep(next - now);
    }
}

/// Convert a 16-bit LPCM sample to an 8-bit µ-law value.
pub fn linear_to_ulaw(sample: i16) -> u8 {
    let mut pcm_value = sample;
    let sign = (pcm_value >> 8) & 0x80;
    if sign != 0 {
        pcm_value = -pcm_value;
    }
    if pcm_value > 32635 {
        pcm_value = 32635;
    }
    pcm_value += 0x84;
    let mut exponent: i16 = 7;
    let mut mask = 0x4000;
    while pcm_value & mask == 0 {
        exponent -= 1;
        mask >>= 1;
    }
    let manitssa: i16 = (pcm_value >> (exponent + 3)) & 0x0f;
    let ulaw_value = sign | exponent << 4 | manitssa;
    (!ulaw_value) as u8
}
