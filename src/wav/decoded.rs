use crate::parser::error::LgAudioParseErr;

use super::chunk::{data::WavDataChunk, fact::{WavFactChunk, WavFactExt}, fmt::{WavFmtChunk, WavFormatType}, LgWavRaw};

pub trait LgWavSampleType {}
impl LgWavSampleType for u8 {}

#[derive(Default, Clone)]
pub struct LgWavDecoded<T: WavFactExt> {
    pub fmt: WavFmtChunk,
    pub fact: Option<WavFactChunk<T>>,
    pub duration: f32,
    pub samples: Vec<i32>
}
impl<T: WavFactExt> TryFrom<LgWavRaw<T>> for LgWavDecoded<T> {
    type Error = String;

    fn try_from(wav_raw: LgWavRaw<T>) -> Result<Self, Self::Error> {
        let num_samples = wav_raw.data.len() 
                / (wav_raw.fmt.bits_per_sample / 8) as usize
                / wav_raw.fmt.number_channels as usize;

        //TODO: This is waaaay to slow, like waay to slow. (even with rayon)
        let samples = wav_raw.data.data
            .chunks_exact(wav_raw.fmt.bits_per_sample as usize / 8)
            .filter_map(|byte| decode_bytes(byte, wav_raw.fmt.fmt_tag).ok())
            .collect();

        let result = Self {
            fmt: wav_raw.fmt,
            fact: wav_raw.fact,
            duration: num_samples as f32 / wav_raw.fmt.samples_per_sec as f32,
            samples,
        };
        
        Ok(result)
    }
}
impl<T: WavFactExt> TryInto<Vec<u8>> for LgWavDecoded<T> {
    type Error = LgAudioParseErr;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let data_bytes = self.encode_samples()?;
        let raw = LgWavRaw {
            fmt: self.fmt,
            fact: self.fact,
            data: WavDataChunk {
                ck_size: data_bytes.len(),
                data: data_bytes,
            },
        };

        Ok(raw.to_bytes())
    }
}
impl<T: WavFactExt> LgWavDecoded<T> {
    pub fn to_bytes(self) -> Result<Vec<u8>, LgAudioParseErr> {
        Ok(self.try_into()?)
    }

    fn encode_samples(&self) -> Result<Vec<u8>, LgAudioParseErr> {
        let mut result = Vec::with_capacity(
            self.samples.len() 
            * (self.fmt.bits_per_sample / 8) as usize
            * self.fmt.number_channels as usize
        );
        for sample in &self.samples {
            result.append(&mut encode_data(*sample, self.fmt.fmt_tag, self.fmt.bits_per_sample)?)
        }
        
        Ok(result)
    }
}
#[inline]
fn encode_data(data: i32, format: WavFormatType, bit_depth: u16) -> Result<Vec<u8>, LgAudioParseErr> {
    let mut result = Vec::default();
    let data = data;

    match format {
        WavFormatType::WAVE_FORMAT_PCM => match bit_depth / 8 {
            3 => result.append(&mut vec![
                (data & 0xFF) as u8,
                ((data >> 8) & 0xFF) as u8,
                ((data >> 16) & 0xFF) as u8,
            ]),
            4 => result.append(&mut data.to_le_bytes().to_vec()),
            blen => return Err(LgAudioParseErr::PARSE(std::format!("Invalid bytes_per_sample: {blen}"))),
        },
        WavFormatType::WAVE_FORMAT_IEEE_FLOAT => result.append(&mut data.to_le_bytes().to_vec()),
        WavFormatType::WAVE_FORMAT_ALAW => todo!(),
        WavFormatType::WAVE_FORMAT_MULAW => todo!(),
        WavFormatType::WAVE_FORMAT_EXTENSIBLE => todo!(),
    };
    
    Ok(result)
}
#[inline]
fn decode_bytes(bytes: &[u8], format: WavFormatType) -> Result<i32, LgAudioParseErr> {
    Ok(match format {
        WavFormatType::WAVE_FORMAT_PCM => {
            match bytes.len() {
                1 => (bytes[0] - 128) as i32, // 8-bit (unsigned)
                2 => i16::from_le_bytes([bytes[0], bytes[1]]) as i32,
                3 => (bytes[2] as i32) << 16 // 24-bit signed
                | (bytes[1] as i32) << 8
                | (bytes[0] as i32)
                | ((bytes[2] & 0x80) as i32) << 24,
                4 => i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),

                blen => return Err(LgAudioParseErr::PARSE(std::format!("Invalid bytes_per_sample: {blen}"))),
            }
        },
        WavFormatType::WAVE_FORMAT_IEEE_FLOAT => i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        WavFormatType::WAVE_FORMAT_ALAW => todo!(),
        WavFormatType::WAVE_FORMAT_MULAW => todo!(),
        WavFormatType::WAVE_FORMAT_EXTENSIBLE => todo!(),
    })
}