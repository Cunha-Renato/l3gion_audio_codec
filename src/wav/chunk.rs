use super::WavFmtTag;

pub(super) enum WavChunks {
    FMT(WavFmt),
    FACT,
    DATA(u32),
}

#[derive(Default, Debug, Clone, Copy)]
pub struct WavFmt {
    pub format: WavFmtTag,
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
}