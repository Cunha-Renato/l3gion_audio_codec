use crate::{parser::error::LgAudioParseErr, primitive_tool::FromLeBytesSlice, reader::LgVecReader};

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
impl std::fmt::Display for WavFormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            WavFormatType::WAVE_FORMAT_PCM => "WAVE_FORMAT_PCM",
            WavFormatType::WAVE_FORMAT_IEEE_FLOAT => "WAVE_FORMAT_IEEE_FLOAT",
            WavFormatType::WAVE_FORMAT_ALAW => "WAVE_FORMAT_ALAW",
            WavFormatType::WAVE_FORMAT_MULAW => "WAVE_FORMAT_MULAW",
            WavFormatType::WAVE_FORMAT_EXTENSIBLE => "WAVE_FORMAT_EXTENSIBLE",
        };

        write!(f, "{msg}")
    }
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
    pub fn read_bytes(ck_size: usize, bytes: &mut LgVecReader<u8>) -> Result<Self, LgAudioParseErr> {
        if ck_size < COMMON_SIZE 
            || ck_size > MAX_SIZE 
        { 
            return Err(LgAudioParseErr::PARSE("Invalid WAVE fmt_chunk size!".to_string()));
        }
        
        // Adding padding
        let bytes = if ck_size < MAX_SIZE {
            let mut bytes = bytes.read_quantity(ck_size)?.to_vec();
            bytes.append(&mut vec![0u8; MAX_SIZE - ck_size]);

            &mut bytes.into()
        }
        else { bytes };

        Ok(Self {
            ck_size,
            fmt_tag: WavFormatType::from(u16::first_from_le_bytes(bytes.read_quantity(2)?)),
            number_channels: u16::first_from_le_bytes(bytes.read_quantity(2)?),
            samples_per_sec: u32::first_from_le_bytes(bytes.read_quantity(4)?),
            avg_bytes_per_sec: u32::first_from_le_bytes(bytes.read_quantity(4)?),
            block_align: u16::first_from_le_bytes(bytes.read_quantity(2)?),
            bits_per_sample: u16::first_from_le_bytes(bytes.read_quantity(2)?),
            cb_size: u16::first_from_le_bytes(bytes.read_quantity(2)?),
            valid_bits_per_sample: u16::first_from_le_bytes(bytes.read_quantity(2)?),
            channel_mask: u32::first_from_le_bytes(bytes.read_quantity(4)?),
            sub_format: u128::first_from_le_bytes(bytes.read_quantity(16)?),
        })
    }
    
    pub fn to_bytes(self) -> Vec<u8> {
        self.into()
    }
}
impl Into<Vec<u8>> for WavFmtChunk {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.ck_size + 4);

        let fmt_tag: u16 = self.fmt_tag.into();
        result.extend((self.ck_size as u32).to_le_bytes());
        result.extend(fmt_tag.to_le_bytes());
        result.extend(self.number_channels.to_le_bytes());
        result.extend(self.samples_per_sec.to_le_bytes());
        result.extend(self.avg_bytes_per_sec.to_le_bytes());
        result.extend(self.block_align.to_le_bytes());
        result.extend(self.bits_per_sample.to_le_bytes());
        result.extend(self.cb_size.to_le_bytes());
        result.extend(self.valid_bits_per_sample.to_le_bytes());
        result.extend(self.channel_mask.to_le_bytes());
        result.extend(self.sub_format.to_le_bytes());
        result.truncate(self.ck_size + 4);
        
        result
    }
}