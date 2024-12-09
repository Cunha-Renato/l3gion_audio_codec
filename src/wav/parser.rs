use crate::byte_conversion::LeByteConversion;
use crate::primitive_tool::FromLeBytesSlice;
use crate::reader::LgVecReader;
use crate::{parser::{error::LgAudioParseErr, LgAudioFileParser}, reader, wav::chunk::{data::WavDataChunk, fact::WavFactChunk, fmt::WavFmtChunk}};
use super::chunk::fact::WavFactExt;
use super::LgWav;

const FMT: &str =  "fmt ";
const FACT: &str = "fact";
const DATA: &str = "data";

#[derive(Debug, Clone)]
pub struct LgWavRaw<T>
where 
    T: WavFactExt
{
    pub fmt: WavFmtChunk,
    pub fact: Option<WavFactChunk<T>>,
    pub data: WavDataChunk,
}
impl std::default::Default for LgWavRaw<Vec<u8>> {
    fn default() -> Self {
        Self {
            fmt: WavFmtChunk::default(),
            fact: None,
            data: WavDataChunk::default(),
        }
    }
}
impl<T> LgAudioFileParser for LgWavRaw<T> 
where 
    T: WavFactExt,
{
    type R = Result<Self, LgAudioParseErr>;

    fn parse(&mut self, path: impl AsRef<str>) -> Result<Self, LgAudioParseErr> {
        let mut bytes: LgVecReader<u8> = reader::read_file(path, "wav, wave")?.into();

        if !self.header_valid(bytes.read_quantity(12)?) { return Err(LgAudioParseErr::PARSE("Invalid WAV header!".to_string())); }

        Ok(Self::parse_chunks(bytes)?)
    }
}
impl<T> LgWavRaw<T>
where 
    T: WavFactExt,
{
    pub fn new() -> Self {
        Self {
            fmt: WavFmtChunk::default(),
            fact: None,
            data: WavDataChunk::default(),
        }
    }
    pub fn fact_ext<U>(self) -> LgWavRaw<U>
    where 
        U: WavFactExt
    {
        LgWavRaw::<U>::new()
    }

    pub fn decode_f64(self) -> Result<LgWav<T, f64>, Box<dyn std::error::Error>> {
        Ok(LgWav::<T, f64>::decode_f64(self.fmt, self.fact, self.data)?)
    }
    
    pub fn decode_with<U: LeByteConversion, F>(self, func: F) -> Result<LgWav<T, U>, Box<dyn std::error::Error>> 
    where F: FnMut(&[u8], &WavFmtChunk, &Option<WavFactChunk<T>>) -> Option<U>
    {
        Ok(LgWav::decode_with(self.fmt, self.fact, self.data, func)?)
    }

    fn header_valid(&self, bytes: &[u8]) -> bool {
        match (
            &*String::from_utf8_lossy(&bytes[..4]), 
            &*String::from_utf8_lossy(&bytes[8..])
        ) {
            ("RIFF", "WAVE") => true,
            _ => true,
        }
    }
    
    // Returns all the valid chunks (id, data).
    fn parse_chunks(mut bytes: LgVecReader<u8>) -> Result<Self, LgAudioParseErr> {
        let mut fmt = WavFmtChunk::default();
        let mut fact = None;
        let mut data = WavDataChunk::default();

        while !bytes.reach_end() {
            // Parsing the chunk id and it's size
            let ck_id = String::from_utf8_lossy(bytes.read_quantity(4)?).to_string();
            let ck_size = u32::first_from_le_bytes(bytes.read_quantity(4)?) as usize;

            match ck_id.as_str() {
                FMT => fmt = WavFmtChunk::read_bytes(ck_size, &mut bytes)?,
                FACT => fact = if ck_size >= 4 {
                    Some(WavFactChunk::<T>::read_bytes(ck_size, &mut bytes)?)
                }
                else { 
                    bytes.skip_quantity(ck_size)?;
                    None
                },
                DATA => {
                    data = WavDataChunk::read_bytes(ck_size, &mut bytes)?;

                    // More info could be stored in the file, but we don't care, so as soon as we
                    // see the data chunk we end parsing
                    break;
                },
                _ => (),
            }
        }

        Ok(Self {
            fmt,
            fact,
            data,
        })
    }
}