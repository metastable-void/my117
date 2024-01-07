
use std::io::Write;
use std::time::{Duration, Instant};

use my117::chrono::prelude::*;
use my117::sleep_until;
use my117::sound;
use my117::time::Time;
use my117::voice::Synthesizable;
use my117::linear_to_ulaw;

const SAMPLE_RATE: usize = 8000;
const FRAME_SIZE: usize = SAMPLE_RATE / 40;

#[derive(Debug, Clone)]
struct Jiho {
    voice: Vec<i16>,
    sound_byo: &'static [i16],
    sound_jiho: &'static [i16],
    sound_yoho: &'static [i16],
}

impl Jiho {
    fn new() -> Jiho {
        let time = Time::now();
        let voice_parts = time.get_next_voice_parts();
        let voice = voice_parts.synthesize();
        Jiho {
            voice,
            sound_byo: sound::byo(),
            sound_jiho: sound::jiho(),
            sound_yoho: sound::yoho(),
        }
    }

    fn update_voice(&mut self, frame: &Frame) {
        self.voice = frame.get_voice();
    }

    fn get_byo_sound(&self, frame: &Frame) -> [i16; FRAME_SIZE] {
        let mut result = [0i16; FRAME_SIZE];
        let millisecond = frame.millisecond();
        let mut i: usize = 0usize;
        let mut j: usize = (millisecond * 8) as usize;
        while j < self.sound_byo.len() && i < FRAME_SIZE {
            result[i] = self.sound_byo[j];
            i += 1;
            j += 1;
        }
        result
    }

    fn get_jiho_sound(&self, frame: &Frame) -> [i16; FRAME_SIZE] {
        let mut result = [0i16; FRAME_SIZE];
        let millisecond = frame.millisecond();
        let second = (frame.second() % 10) as u32;
        let millisecond = second * 1000 + millisecond;
        let mut i: usize = 0usize;
        let mut j: usize = (millisecond * 8) as usize;
        while j < self.sound_jiho.len() && i < FRAME_SIZE {
            result[i] = self.sound_jiho[j];
            i += 1;
            j += 1;
        }
        result
    }

    fn get_yoho_sound(&self, frame: &Frame) -> [i16; FRAME_SIZE] {
        let mut result = [0i16; FRAME_SIZE];
        let millisecond = frame.millisecond();
        let second = frame.second() as u32;
        if second == 27 || second == 28 || second == 29 || second == 57 || second == 58 || second == 59 {
            let mut i: usize = 0usize;
            let mut j: usize = (millisecond * 8) as usize;
            while j < self.sound_yoho.len() && i < FRAME_SIZE {
                result[i] = self.sound_yoho[j];
                i += 1;
                j += 1;
            }
        }
        result
    }

    fn get_voice_sound(&self, frame: &Frame) -> [i16; FRAME_SIZE] {
        let mut result = [0i16; FRAME_SIZE];
        let millisecond = frame.millisecond() as i32;
        let second = (frame.second() % 10) as i32;
        let millisecond = second * 1000 + millisecond;
        let mut i: usize = 0usize;
        let mut j: isize = ((millisecond - 500) * 8) as isize;
        while j < self.voice.len() as isize && i < FRAME_SIZE {
            if j >= 0 {
                let j: usize = j as usize;
                result[i] = self.voice[j];
            }
            i += 1;
            j += 1;
        }
        result
    }

}

#[derive(Debug, Clone)]
struct Frame {
    time_to_second: Time,
    milliseconds: u32, // multiple of 25
}

impl Frame {
    fn next() -> Frame {
        let datetime = Local::now();
        let milliseconds = datetime.timestamp_subsec_millis();
        let remaining = milliseconds % 25;
        let add = 25 - remaining;
        let add_duration = Duration::from_millis(add.into());
        let datetime = datetime + add_duration;
        let time_to_second: Time = datetime.into();
        Frame {
            time_to_second,
            milliseconds: datetime.timestamp_subsec_millis(),
        }
    }

    fn get_voice(&self) -> Vec<i16> {
        let voice_parts = self.time_to_second.get_next_voice_parts();
        voice_parts.synthesize()
    }

    fn second(&self) -> u8 {
        self.time_to_second.second()
    }

    fn millisecond(&self) -> u32 {
        self.milliseconds
    }
}

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut jiho = Jiho::new();
    let start = Instant::now();
    let mut next_frame = start;
    loop {
        let frame = Frame::next();
        let second = frame.second();
        let millisecond = frame.millisecond();
        if second % 10 == 0 && millisecond == 0 {
            jiho.update_voice(&frame);
        }
        let mut buf: [i16; FRAME_SIZE] = [0; FRAME_SIZE];
        let byo_sound = jiho.get_byo_sound(&frame);
        let jiho_sound = jiho.get_jiho_sound(&frame);
        let yoho_sound = jiho.get_yoho_sound(&frame);
        let voice_sound = jiho.get_voice_sound(&frame);
        for i in 0..FRAME_SIZE {
            buf[i] = byo_sound[i] + jiho_sound[i] + yoho_sound[i] + voice_sound[i];
        }
        let bytes = buf.iter().map(|&x| linear_to_ulaw(x)).collect::<Vec<u8>>();
        if let Err(_) = stdout.write_all(&bytes) {
            break;
        }
        next_frame += Duration::from_millis(25);
        sleep_until(next_frame);
    }
}
