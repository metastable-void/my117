
use std::io;
use std::io::Write;
use my117::time::Time;
use my117::voice::Synthesizable;

const WAV_FMT_SUBCHUNK_SIZE: u32 = 16;
const WAV_FORMAT_PCM: u16 = 1;
const WAV_BITS_PER_SAMPLE: u16 = 16;
const WAV_SAMPLE_RATE: u32 = 48000;
const WAV_CHANNELS: u16 = 1;

fn main() {
    let time = Time::now();
    let voice_parts = time.get_next_voice_parts();
    let buf = voice_parts.synthesize();
    let bytes: Vec<u8> = buf.iter().flat_map(|&x| x.to_le_bytes().to_vec()).collect();
    let chunk_size: u32 = bytes.len() as u32 + 36;
    let byte_rate: u32 = WAV_SAMPLE_RATE * u32::from(WAV_BITS_PER_SAMPLE) / 8;
    let block_align: u16 = WAV_BITS_PER_SAMPLE / 8;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(b"RIFF").unwrap();
    stdout.write_all(&chunk_size.to_le_bytes()).unwrap();
    stdout.write_all(b"WAVEfmt ").unwrap();
    stdout.write_all(&WAV_FMT_SUBCHUNK_SIZE.to_le_bytes()).unwrap();
    stdout.write_all(&WAV_FORMAT_PCM.to_le_bytes()).unwrap();
    stdout.write_all(&WAV_CHANNELS.to_le_bytes()).unwrap();
    stdout.write_all(&WAV_SAMPLE_RATE.to_le_bytes()).unwrap();
    stdout.write_all(&byte_rate.to_le_bytes()).unwrap();
    stdout.write_all(&block_align.to_le_bytes()).unwrap();
    stdout.write_all(&WAV_BITS_PER_SAMPLE.to_le_bytes()).unwrap();
    stdout.write_all(b"data").unwrap();
    stdout.write_all(&(bytes.len() as u32).to_le_bytes()).unwrap();
    stdout.write_all(&bytes).unwrap();
}
