use std::marker::PhantomData;

use crate::{parser::{error::LgAudioParseErr, LgAudioFileParser}, reader, wav::chunk::{data::WavDataChunk, fact::WavFactChunk, fmt::WavFmtChunk}};

use super::chunk::LgWavChunks;

const FMT: &str = "fmt ";
const FACT: &str = "fact";
const DATA: &str = "data";

#[derive(Debug, Clone, Copy)]
pub struct LgWavParser<T>
where 
    T: TryFrom<Vec<u8>>,
    T: Into<Vec<u8>>,
    T: std::fmt::Debug,
    T: Clone,
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
    T: TryFrom<Vec<u8>>,
    T: Into<Vec<u8>>,
    T: std::fmt::Debug,
    T: Clone,
    <T as TryFrom<Vec<u8>>>::Error: std::error::Error + 'static
{
    type R = Result<LgWavChunks<T>, LgAudioParseErr>;

    fn parse(&mut self, path: impl AsRef<str>) -> Result<LgWavChunks<T>, LgAudioParseErr> {
        let bytes = reader::read_file(path, "wav, wave")?;
        let mut result = LgWavChunks::<T>::default();

        if self.header_valid(&bytes[..12]) {
            let chunks = self.parse_chunks(bytes);
            for (ck_id, ck_data) in chunks {
                match ck_id.as_str() {
                    FMT => result.fmt = WavFmtChunk::try_from(ck_data).unwrap(),
                    FACT => result.fact = Some(WavFactChunk::<T>::try_from(ck_data).unwrap()),
                    DATA => result.data = WavDataChunk::from(ck_data),
                    _ => (),
                }
            }
        }
        
        Ok(result)
    }
}
impl<T> LgWavParser<T>
where 
    T: TryFrom<Vec<u8>>,
    T: Into<Vec<u8>>,
    T: std::fmt::Debug,
    T: Clone
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn fact_ext<U>(self) -> LgWavParser<U>
    where 
        U: TryFrom<Vec<u8>>,
        U: Into<Vec<u8>>,
        U: std::fmt::Debug,
        U: Clone 
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
    fn parse_chunks(&self, bytes: Vec<u8>) -> Vec<(String, Vec<u8>)> {
        let mut result = Vec::default();
        let mut cursor = 12;

        while cursor < bytes.len() {
            // Parsing the chunk id and it's size
            let ck_id = String::from_utf8_lossy(&bytes[cursor..cursor + 4]);
            let ck_size = u32::from_le_bytes([
                bytes[cursor + 4], 
                bytes[cursor + 5], 
                bytes[cursor + 6], 
                bytes[cursor + 7]
            ]);
            cursor += 8;

            // The rest of the chunk is the chunk data.
            result.push((
                ck_id.to_string(), 
                bytes[cursor..cursor + ck_size as usize].to_vec()
            ));

            // More info could be stored in the file, but we don't care, so as soon as we
            // see the data chunk we end parsing
            if &ck_id.to_string() == DATA { return result; }

            cursor += ck_size as usize;
        }

        result
    }
}