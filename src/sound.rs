
use std::sync::OnceLock;

static SOUND_BYO: &'static [u8] = include_bytes!("../sounds/byo.s16le");
static SOUND_JIHO: &'static [u8] = include_bytes!("../sounds/jiho.s16le");
static SOUND_YOHO: &'static [u8] = include_bytes!("../sounds/yoho.s16le");

static CELL_BYO: OnceLock<Vec<i16>> = OnceLock::new();
static CELL_JIHO: OnceLock<Vec<i16>> = OnceLock::new();
static CELL_YOHO: OnceLock<Vec<i16>> = OnceLock::new();

pub fn byo() -> &'static [i16] {
    CELL_BYO.get_or_init(|| {
        SOUND_BYO
            .chunks_exact(2)
            .map(|x| i16::from_le_bytes([x[0], x[1]]))
            .collect()
    })
}

pub fn jiho() -> &'static [i16] {
    CELL_JIHO.get_or_init(|| {
        SOUND_JIHO
            .chunks_exact(2)
            .map(|x| i16::from_le_bytes([x[0], x[1]]))
            .collect()
    })
}

pub fn yoho() -> &'static [i16] {
    CELL_YOHO.get_or_init(|| {
        SOUND_YOHO
            .chunks_exact(2)
            .map(|x| i16::from_le_bytes([x[0], x[1]]))
            .collect()
    })
}
