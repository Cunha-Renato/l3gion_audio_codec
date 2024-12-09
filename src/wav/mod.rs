pub mod parser;
pub mod chunk;
pub mod alaw;
pub mod mulaw;

use crate::{byte_conversion::LeByteConversion, parser::error::LgAudioParseErr};
use alaw::{encode_alaw, ALAW_DECOMPRESS_TABLE};
use mulaw::{encode_mulaw, MULAW_DECOMPRESS_TABLE};
use chunk::{data::WavDataChunk, fact::{WavFactChunk, WavFactExt}, fmt::{WavFmtChunk, WavFormatType}};

pub trait LgWavSampleType {}
impl LgWavSampleType for u8 {}

#[derive(Default, Clone)]
pub struct LgWav<T: WavFactExt, D: LeByteConversion> {
    pub fmt: WavFmtChunk,
    pub fact: Option<WavFactChunk<T>>,
    pub duration: f32,
    pub samples: Vec<D>
}
impl<T: WavFactExt, D: LeByteConversion> std::fmt::Debug for LgWav<T, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LgWav")
            .field("fmt", &self.fmt)
            .field("fact", &self.fact)
            .field("duration", &self.duration)
            .field("samples_len", &self.samples.len())
            .finish()
    }
}
impl<T: WavFactExt, D: LeByteConversion> LgWav<T, D> {
    /// This function just does the std ::from_le_bytes().
    /// TODO: If you chose to have a different type for the samples use decode_as(), if you have your own format use decode_with().
    fn decode_with<F>(
        fmt: WavFmtChunk,
        fact: Option<WavFactChunk<T>>,
        mut data: WavDataChunk,
        mut func: F,
    ) -> Result<LgWav<T, D>, Box<dyn std::error::Error>> 
    where F: FnMut(&[u8], &WavFmtChunk, &Option<WavFactChunk<T>>) -> Option<D>,
    {
        let num_samples = num_samples(data.len(), &fmt);
        let data = std::mem::take(&mut data.data);

        //TODO: This is waaaay to slow, like waay to slow. (even with rayon)
        // maybe do a C like loop with index, maybe it's the chunks_exact? idk.
        let samples = data.chunks_exact(fmt.bits_per_sample as usize / 8)
            .filter_map(|bytes| func(bytes, &fmt, &fact))
            .collect::<Vec<_>>();

        if samples.len() != num_samples * fmt.number_channels as usize {
            return Err("Failed to decode all samples!".into());
        }

        Ok(LgWav::<T, D> {
            fmt,
            fact,
            duration: num_samples as f32 / fmt.samples_per_sec as f32,
            samples,
        })
    }
    
    pub fn decode_f64(
        fmt: WavFmtChunk,
        fact: Option<WavFactChunk<T>>,
        data: WavDataChunk,
    ) -> Result<LgWav<T, f64>, Box<dyn std::error::Error>>
    {
        LgWav::<T, f64>::decode_with(fmt, fact, data, |bytes, fmt, _| decode_bytes(bytes, fmt.fmt_tag, fmt.bits_per_sample).ok())
    }
    
    pub fn encode_with<F, E>(&self, func: F) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where 
        F: Fn(&D, &WavFmtChunk, &Option<WavFactChunk<T>>) -> Result<Vec<u8>, E>,
        E: std::error::Error + 'static,
    {
        let mut result = Vec::default();

        for sample in &self.samples {
            result.append(&mut func(sample, &self.fmt, &self.fact)?)
        }
        
        Ok(result)
    }

    pub fn to_bytes_with<F, E>(self, func: F) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
        F: Fn(&D, &WavFmtChunk, &Option<WavFactChunk<T>>) -> Result<Vec<u8>, E>,
        E: std::error::Error + 'static,
    {
        let data_bytes = self.encode_with(func)?;
        let data = WavDataChunk {
            ck_size: data_bytes.len(),
            data: data_bytes
        };
        
        Ok(to_bytes(self.fmt, self.fact, data))
    }
}
impl<T: WavFactExt> LgWav<T, f64> {
    pub fn to_bytes_f64(self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(self.to_bytes_with(encode_data)?)
    }

    pub fn encode_data_f64(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(self.encode_with(encode_data)?)
    }
}

pub fn to_bytes<T: WavFactExt>(fmt: WavFmtChunk, fact: Option<WavFactChunk<T>>, data: WavDataChunk) -> Vec<u8> {
    let mut chunks = Vec::default(); 

    // Fmt chunk
    chunks.extend(b"fmt ");
    chunks.extend(fmt.to_bytes());

    // Fact chunk
    if let Some(fact) = fact {
        chunks.extend(b"fact");
        chunks.extend(fact.to_bytes());
    }

    // Data chunk
    chunks.extend(b"data");
    chunks.extend(data.to_bytes());
    
    // Header
    let mut result = Vec::with_capacity(chunks.len() + 12);
    result.extend(b"RIFF");
    result.extend((chunks.len() as u32 + 4).to_le_bytes());
    result.extend(b"WAVE");
    result.extend(std::mem::take(&mut chunks));
    
    result
}

fn num_samples(data_len: usize, fmt: &WavFmtChunk) -> usize {
    data_len
    / (fmt.bits_per_sample / 8) as usize
    / fmt.number_channels as usize
}

#[inline]
pub fn decode_bytes(bytes: &[u8], fmt_type: WavFormatType, bits_per_sample: u16) -> Result<f64, Box<dyn std::error::Error>> {
    Ok(match fmt_type {
        WavFormatType::WAVE_FORMAT_PCM => match bits_per_sample {
            16 => i16::from_le_bytes([bytes[0], bytes[1]]) as f64,
            24 => decode_24bit_i32(bytes) as f64,
            32 => i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64,
            bps if bps <= 8 => bytes[0] as f64,

            _ => return Err(std::format!("Not supported: {fmt_type} with {bits_per_sample} bits_per_sample!").into())
        },
        WavFormatType::WAVE_FORMAT_IEEE_FLOAT => match bits_per_sample {
            32 => f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64,
            64 => f64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]),

            _ => return Err(std::format!("Not supported: {fmt_type} with {bits_per_sample} bits_per_sample!").into())
        },
        WavFormatType::WAVE_FORMAT_ALAW => ALAW_DECOMPRESS_TABLE[bytes[0] as usize] as f64,
        WavFormatType::WAVE_FORMAT_MULAW => MULAW_DECOMPRESS_TABLE[bytes[0] as usize] as f64,
        WavFormatType::WAVE_FORMAT_EXTENSIBLE => return Err(std::format!("Not supported: {fmt_type} with default decode function!").into()),
    })
}

#[inline]
pub fn encode_data<T: WavFactExt>(data: &f64, fmt: &WavFmtChunk, _: &Option<WavFactChunk<T>>) -> Result<Vec<u8>, LgAudioParseErr> {
    let data_i32 = *data as i32;
    let fmt_type = fmt.fmt_tag;
    let bits_per_sample = fmt.bits_per_sample;

    Ok(match fmt_type {
        WavFormatType::WAVE_FORMAT_PCM => match bits_per_sample {
            16 => (data_i32 as i16).to_le_bytes().to_vec(),
            24 => encode_24bit_i32(data_i32).to_vec(),
            32 => data_i32.to_le_bytes().to_vec(),

            bps if bps <= 8 => vec![data_i32 as u8],

            _ => return Err(std::format!("Not supported: {fmt_type} with {bits_per_sample} bits_per_sample!").into())
        },
        WavFormatType::WAVE_FORMAT_IEEE_FLOAT => match bits_per_sample {
            32 => (*data as f32).to_le_bytes().to_vec(),
            64 => data.to_le_bytes().to_vec(),

            _ => return Err(std::format!("Not supported: {fmt_type} with {bits_per_sample} bits_per_sample!").into())
        },
        WavFormatType::WAVE_FORMAT_ALAW => vec![encode_alaw(*data as i16)],
        WavFormatType::WAVE_FORMAT_MULAW => vec![encode_mulaw(*data as i16)],
        WavFormatType::WAVE_FORMAT_EXTENSIBLE => return Err(std::format!("Not supported: {fmt_type} with default encode function!").into()),
    })
}

#[inline]
const fn decode_24bit_i32(bytes: &[u8]) -> i32 {
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], if bytes[2] & 0x80 != 0 { 0xFF } else { 0x00 }])
}

#[inline]
const fn encode_24bit_i32(data: i32) -> [u8; 3] {
    let bytes = data.to_le_bytes();

    [bytes[0], bytes[1], bytes[2]]
}