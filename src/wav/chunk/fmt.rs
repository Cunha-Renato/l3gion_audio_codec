use crate::primitive_tool::FromLeBytesSlice;

const COMMON_SIZE: usize = 14;
const SPECIFIC_SIZE: usize = 26;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WavFormatType {
    WAVE_FORMAT_PCM,
    WAVE_FORMAT_IEEE_FLOAT,
    WAVE_FORMAT_ALAW,
    WAVE_FORMAT_MULAW,
    WAVE_FORMAT_EXTENSIBLE,
}
impl From<u16> for WavFormatType {
    fn from(code: u16) -> Self {
        match code {
            0x0001 => Self::WAVE_FORMAT_PCM,
            0x0003 => Self::WAVE_FORMAT_IEEE_FLOAT,
            0x0006 => Self::WAVE_FORMAT_ALAW,
            0x0007 => Self::WAVE_FORMAT_MULAW,
            0xFFFE => Self::WAVE_FORMAT_EXTENSIBLE,
            _ => panic!("Wrong wav_fmt_tag code!"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WavFmtCommon {
    pub fmt_tag: WavFormatType,
    pub number_channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
}
impl From<&[u8]> for WavFmtCommon {
    fn from(bytes: &[u8]) -> Self {
        Self {
            fmt_tag: WavFormatType::from(u16::first_from_le_bytes(&bytes[..2])),
            number_channels: u16::first_from_le_bytes(&bytes[2..4]),
            samples_per_sec: u32::first_from_le_bytes(&bytes[4..8]),
            avg_bytes_per_sec: u32::first_from_le_bytes(&bytes[8..12]),
            block_align: u16::first_from_le_bytes(&bytes[12..]),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WavFmtSpecific {
    pub bits_per_sample: u16,
    pub cb_size: u16,
    pub valid_bits_per_sample: u16,
    pub channel_mask: u32,
    pub sub_format: u128,
}
impl From<&[u8]> for WavFmtSpecific {
    fn from(bytes: &[u8]) -> Self {
        Self {
            bits_per_sample: u16::first_from_le_bytes(&bytes[..2]),
            cb_size: u16::first_from_le_bytes(&bytes[2..4]),
            valid_bits_per_sample: u16::first_from_le_bytes(&bytes[4..6]),
            channel_mask: u32::first_from_le_bytes(&bytes[6..10]),
            sub_format: u128::first_from_le_bytes(&bytes[10..]),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WavFmtChunk {
    pub common: WavFmtCommon,
    pub specific: WavFmtSpecific,
}
impl WavFmtChunk {
    /// This function assumes that the ck_id is not present in the bytes.
    pub fn from(ck_size: usize, bytes: &[u8]) -> Option<Self> {
        if bytes.len() < COMMON_SIZE 
            || ck_size < COMMON_SIZE
            || ck_size != bytes.len()
        { 
            return None; 
        }

        let common = WavFmtCommon::from(&bytes[..COMMON_SIZE]);
        
        // Specific
        let mut padding = vec![0u8; SPECIFIC_SIZE - (ck_size - COMMON_SIZE)];
        let mut specific_data = bytes[COMMON_SIZE..].to_vec();
        specific_data.append(&mut padding);

        let specific = WavFmtSpecific::from(specific_data.as_slice());

        Some(Self {
            common,
            specific,
        })
    }
}