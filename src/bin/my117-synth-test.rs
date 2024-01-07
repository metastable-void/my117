
use std::io;
use std::io::Write;
use my117::time::Time;
use my117::voice::Synthesizable;

fn main() {
    let time = Time::now();
    let voice_parts = time.get_next_voice_parts();
    let buf = voice_parts.synthesize();
    let bytes: Vec<u8> = buf.iter().flat_map(|&x| x.to_le_bytes().to_vec()).collect();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(&bytes).unwrap();
}
