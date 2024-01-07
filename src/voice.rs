//! Voice data in PCM s16le format.
//! Also contains utility functions to synthesize PCM data from parts.

static VOICE_GOZEN: &'static [u8] = include_bytes!("../voices/gozen.s16le");
static VOICE_GOGO: &'static [u8] = include_bytes!("../voices/gogo.s16le");
static VOICE_SHOGOWO: &'static [u8] = include_bytes!("../voices/shogowo.s16le");

static VOICE_0JI: &'static [u8] = include_bytes!("../voices/0ji.s16le");
static VOICE_1JI: &'static [u8] = include_bytes!("../voices/1ji.s16le");
static VOICE_2JI: &'static [u8] = include_bytes!("../voices/2ji.s16le");
static VOICE_3JI: &'static [u8] = include_bytes!("../voices/3ji.s16le");
static VOICE_4JI: &'static [u8] = include_bytes!("../voices/4ji.s16le");
static VOICE_5JI: &'static [u8] = include_bytes!("../voices/5ji.s16le");
static VOICE_6JI: &'static [u8] = include_bytes!("../voices/6ji.s16le");
static VOICE_7JI: &'static [u8] = include_bytes!("../voices/7ji.s16le");
static VOICE_8JI: &'static [u8] = include_bytes!("../voices/8ji.s16le");
static VOICE_9JI: &'static [u8] = include_bytes!("../voices/9ji.s16le");
static VOICE_10JI: &'static [u8] = include_bytes!("../voices/10ji.s16le");
static VOICE_11JI: &'static [u8] = include_bytes!("../voices/11ji.s16le");

static VOICE_10: &'static [u8] = include_bytes!("../voices/10.s16le");
static VOICE_20: &'static [u8] = include_bytes!("../voices/20.s16le");
static VOICE_30: &'static [u8] = include_bytes!("../voices/30.s16le");
static VOICE_40: &'static [u8] = include_bytes!("../voices/40.s16le");
static VOICE_50: &'static [u8] = include_bytes!("../voices/50.s16le");

static VOICE_1FUN: &'static [u8] = include_bytes!("../voices/1fun.s16le");
static VOICE_2FUN: &'static [u8] = include_bytes!("../voices/2fun.s16le");
static VOICE_3FUN: &'static [u8] = include_bytes!("../voices/3fun.s16le");
static VOICE_4FUN: &'static [u8] = include_bytes!("../voices/4fun.s16le");
static VOICE_5FUN: &'static [u8] = include_bytes!("../voices/5fun.s16le");
static VOICE_6FUN: &'static [u8] = include_bytes!("../voices/6fun.s16le");
static VOICE_7FUN: &'static [u8] = include_bytes!("../voices/7fun.s16le");
static VOICE_8FUN: &'static [u8] = include_bytes!("../voices/8fun.s16le");
static VOICE_9FUN: &'static [u8] = include_bytes!("../voices/9fun.s16le");

static VOICE_10FUN: &'static [u8] = include_bytes!("../voices/10fun.s16le");
static VOICE_20FUN: &'static [u8] = include_bytes!("../voices/20fun.s16le");
static VOICE_30FUN: &'static [u8] = include_bytes!("../voices/30fun.s16le");
static VOICE_40FUN: &'static [u8] = include_bytes!("../voices/40fun.s16le");
static VOICE_50FUN: &'static [u8] = include_bytes!("../voices/50fun.s16le");

static VOICE_CHODO_WO: &'static [u8] = include_bytes!("../voices/chodo-wo.s16le");

static VOICE_10BYO_WO: &'static [u8] = include_bytes!("../voices/10byo-wo.s16le");
static VOICE_20BYO_WO: &'static [u8] = include_bytes!("../voices/20byo-wo.s16le");
static VOICE_30BYO_WO: &'static [u8] = include_bytes!("../voices/30byo-wo.s16le");
static VOICE_40BYO_WO: &'static [u8] = include_bytes!("../voices/40byo-wo.s16le");
static VOICE_50BYO_WO: &'static [u8] = include_bytes!("../voices/50byo-wo.s16le");

static VOICE_OSHIRASESHIMASU: &'static [u8] = include_bytes!("../voices/oshiraseshimasu.s16le");

#[derive(Debug, Clone, Copy)]
pub enum Part {
    VoiceGozen,
    VoiceGogo,
    VoiceShogoWo,
    Voice0ji,
    Voice1ji,
    Voice2ji,
    Voice3ji,
    Voice4ji,
    Voice5ji,
    Voice6ji,
    Voice7ji,
    Voice8ji,
    Voice9ji,
    Voice10ji,
    Voice11ji,
    Voice10,
    Voice20,
    Voice30,
    Voice40,
    Voice50,
    Voice1fun,
    Voice2fun,
    Voice3fun,
    Voice4fun,
    Voice5fun,
    Voice6fun,
    Voice7fun,
    Voice8fun,
    Voice9fun,
    Voice10fun,
    Voice20fun,
    Voice30fun,
    Voice40fun,
    Voice50fun,
    VoiceChodoWo,
    Voice10byoWo,
    Voice20byoWo,
    Voice30byoWo,
    Voice40byoWo,
    Voice50byoWo,
    VoiceOshiraseshimasu,
}

pub trait Synthesizable {
    /// Synthesize PCM 8kHz s16le data
    fn synthesize(&self) -> Vec<i16>;
}

impl Part {
    pub fn get_bytes(self) -> &'static [u8] {
        match self {
            Part::VoiceGozen => VOICE_GOZEN,
            Part::VoiceGogo => VOICE_GOGO,
            Part::VoiceShogoWo => VOICE_SHOGOWO,
            Part::Voice0ji => VOICE_0JI,
            Part::Voice1ji => VOICE_1JI,
            Part::Voice2ji => VOICE_2JI,
            Part::Voice3ji => VOICE_3JI,
            Part::Voice4ji => VOICE_4JI,
            Part::Voice5ji => VOICE_5JI,
            Part::Voice6ji => VOICE_6JI,
            Part::Voice7ji => VOICE_7JI,
            Part::Voice8ji => VOICE_8JI,
            Part::Voice9ji => VOICE_9JI,
            Part::Voice10ji => VOICE_10JI,
            Part::Voice11ji => VOICE_11JI,
            Part::Voice10 => VOICE_10,
            Part::Voice20 => VOICE_20,
            Part::Voice30 => VOICE_30,
            Part::Voice40 => VOICE_40,
            Part::Voice50 => VOICE_50,
            Part::Voice1fun => VOICE_1FUN,
            Part::Voice2fun => VOICE_2FUN,
            Part::Voice3fun => VOICE_3FUN,
            Part::Voice4fun => VOICE_4FUN,
            Part::Voice5fun => VOICE_5FUN,
            Part::Voice6fun => VOICE_6FUN,
            Part::Voice7fun => VOICE_7FUN,
            Part::Voice8fun => VOICE_8FUN,
            Part::Voice9fun => VOICE_9FUN,
            Part::Voice10fun => VOICE_10FUN,
            Part::Voice20fun => VOICE_20FUN,
            Part::Voice30fun => VOICE_30FUN,
            Part::Voice40fun => VOICE_40FUN,
            Part::Voice50fun => VOICE_50FUN,
            Part::VoiceChodoWo => VOICE_CHODO_WO,
            Part::Voice10byoWo => VOICE_10BYO_WO,
            Part::Voice20byoWo => VOICE_20BYO_WO,
            Part::Voice30byoWo => VOICE_30BYO_WO,
            Part::Voice40byoWo => VOICE_40BYO_WO,
            Part::Voice50byoWo => VOICE_50BYO_WO,
            Part::VoiceOshiraseshimasu => VOICE_OSHIRASESHIMASU,
        }
    } 
}

/// Synthesize PCM 8kHz s16le data from parts.
pub fn synthesize(parts: &[Part]) -> Vec<i16> {
    let mut result = Vec::new();
    for part in parts {
        let part = part.get_bytes();
        for i in 0..part.len() / 2 {
            let sample = i16::from_le_bytes([part[i * 2], part[i * 2 + 1]]);
            result.push(sample);
        }
    }
    result
}

impl Synthesizable for Part {
    fn synthesize(&self) -> Vec<i16> {
        synthesize(&[*self])
    }
}

impl<T> Synthesizable for T
where T: AsRef<[Part]>
{
    fn synthesize(&self) -> Vec<i16> {
        synthesize(self.as_ref())
    }
}
