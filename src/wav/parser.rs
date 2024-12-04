use std::marker::PhantomData;
use crate::primitive_tool::FromLeBytesSlice;
use crate::reader::LgVecReader;
use crate::{parser::{error::LgAudioParseErr, LgAudioFileParser}, reader, wav::chunk::{data::WavDataChunk, fact::WavFactChunk, fmt::WavFmtChunk}};
use super::chunk::fact::WavFactExt;
use super::chunk::LgWavRaw;

const FMT: &str = "fmt ";
const FACT: &str = "fact";
const DATA: &str = "data";

#[derive(Debug, Clone, Copy)]
pub struct LgWavParser<T>
where 
    T: WavFactExt
{
    _phantom: PhantomData<T>,
}
impl std::default::Default for LgWavParser<Vec<u8>> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData
        }
    }
}
impl<T> LgAudioFileParser for LgWavParser<T> 
where 
    T: WavFactExt,
{
    type R = Result<LgWavRaw<T>, LgAudioParseErr>;

    fn parse(&mut self, path: impl AsRef<str>) -> Result<LgWavRaw<T>, LgAudioParseErr> {
        let mut bytes: LgVecReader<u8> = reader::read_file(path, "wav, wave")?.into();

        if !self.header_valid(bytes.read_quantity(12)?) { return Err(LgAudioParseErr::PARSE("Invalid WAV header!".to_string())); }

        Ok(self.parse_chunks(bytes)?)
    }
}
impl<T> LgWavParser<T>
where 
    T: WavFactExt,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn fact_ext<U>(self) -> LgWavParser<U>
    where 
        U: WavFactExt
    {
        LgWavParser::<U>::new()
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
    fn parse_chunks(&self, mut bytes: LgVecReader<u8>) -> Result<LgWavRaw<T>, LgAudioParseErr> {
        let mut result = LgWavRaw::default();

        while !bytes.reach_end() {
            // Parsing the chunk id and it's size
            let ck_id = String::from_utf8_lossy(bytes.read_quantity(4)?).to_string();
            let ck_size = u32::first_from_le_bytes(bytes.read_quantity(4)?) as usize;

            match ck_id.as_str() {
                FMT => result.fmt = WavFmtChunk::read_bytes(ck_size, &mut bytes)?,
                FACT => result.fact = if ck_size >= 4 {
                    Some(WavFactChunk::<T>::read_bytes(ck_size, &mut bytes)?)
                }
                else { 
                    bytes.skip_quantity(ck_size)?;
                    None
                },
                DATA => {
                    result.data = WavDataChunk::read_bytes(ck_size, &mut bytes)?;

                    // More info could be stored in the file, but we don't care, so as soon as we
                    // see the data chunk we end parsing
                    break;
                },
                _ => (),
            }
        }

        Ok(result)
    }
}