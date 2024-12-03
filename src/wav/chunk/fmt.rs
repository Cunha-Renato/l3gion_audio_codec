use crate::primitive_tool::FromLeBytesSlice;

const COMMON_SIZE: usize = 14;
const MAX_SIZE: usize = 40;

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum WavFormatType {
    #[default]
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
impl Into<u16> for WavFormatType {
    fn into(self) -> u16 {
        match self {
            WavFormatType::WAVE_FORMAT_PCM => 0x0001,
            WavFormatType::WAVE_FORMAT_IEEE_FLOAT => 0x0003,
            WavFormatType::WAVE_FORMAT_ALAW => 0x0006,
            WavFormatType::WAVE_FORMAT_MULAW => 0x0007,
            WavFormatType::WAVE_FORMAT_EXTENSIBLE => 0xFFFE,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct WavFmtChunk {
    pub ck_size: usize,
    pub fmt_tag: WavFormatType,
    pub number_channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
    pub valid_bits_per_sample: u16,
    pub channel_mask: u32,
    pub sub_format: u128,
}
impl WavFmtChunk {
    pub fn to_vec(self) -> Vec<u8> {
        self.into()
    }
}
impl TryFrom<Vec<u8>> for WavFmtChunk {
    type Error = &'static str;

    fn try_from(mut bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let ck_size = bytes.len();
        if ck_size < COMMON_SIZE 
            || ck_size > MAX_SIZE 
        { 
            return Err("Bytes provided are invalid!");
        }
        
        // Adding padding
        if ck_size < MAX_SIZE {
            let mut padding = vec![0u8; MAX_SIZE - ck_size];
            bytes.append(&mut padding);
        }

        Ok(Self {
            ck_size,
            fmt_tag: WavFormatType::from(u16::first_from_le_bytes(&bytes[..2])),
            number_channels: u16::first_from_le_bytes(&bytes[2..4]),
            samples_per_sec: u32::first_from_le_bytes(&bytes[4..8]),
            avg_bytes_per_sec: u32::first_from_le_bytes(&bytes[8..12]),
            block_align: u16::first_from_le_bytes(&bytes[12..14]),
            bits_per_sample: u16::first_from_le_bytes(&bytes[14..16]),
            cb_size: u16::first_from_le_bytes(&bytes[16..18]),
            valid_bits_per_sample: u16::first_from_le_bytes(&bytes[18..20]),
            channel_mask: u32::first_from_le_bytes(&bytes[20..24]),
            sub_format: u128::first_from_le_bytes(&bytes[24..]),
        })
    }
}
impl Into<Vec<u8>> for WavFmtChunk {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.ck_size);

        let fmt_tag: u16 = self.fmt_tag.into();
        result.extend(fmt_tag.to_le_bytes().iter());
        result.extend(self.number_channels.to_le_bytes().iter());
        result.extend(self.samples_per_sec.to_le_bytes().iter());
        result.extend(self.avg_bytes_per_sec.to_le_bytes().iter());
        result.extend(self.block_align.to_le_bytes().iter());
        result.extend(self.bits_per_sample.to_le_bytes().iter());
        result.extend(self.cb_size.to_le_bytes().iter());
        result.extend(self.valid_bits_per_sample.to_le_bytes().iter());
        result.extend(self.channel_mask.to_le_bytes().iter());
        result.extend(self.sub_format.to_le_bytes().iter());
        result.truncate(self.ck_size);
        
        result
    }
}