use data::WavDataChunk;
use fact::{WavFactChunk, WavFactExt};
use fmt::WavFmtChunk;

use super::decoded::LgWavDecoded;

pub mod fmt;
pub mod fact;
pub mod data;

impl WavFactExt for Vec<u8> {}

#[derive(Debug, Clone)]
pub struct LgWavRaw<T> 
where 
    T: WavFactExt
{
    pub fmt: WavFmtChunk,
    pub fact: Option<WavFactChunk<T>>,
    pub data: WavDataChunk,
}
impl<T> std::default::Default for LgWavRaw<T>
where 
    T: WavFactExt
{
    fn default() -> Self {
        Self { 
            fmt: Default::default(), 
            fact: Default::default(), 
            data: Default::default() 
        }
    }
}
impl<T: WavFactExt> Into<Vec<u8>> for LgWavRaw<T> {
    fn into(self) -> Vec<u8> {
        let mut chunks = Vec::default(); 

        // Fmt chunk
        chunks.extend(b"fmt ");
        chunks.extend(self.fmt.to_bytes());

        // Fact chunk
        if let Some(fact) = self.fact {
            chunks.extend(b"fact");
            chunks.extend(fact.to_bytes());
        }

        // Data chunk
        chunks.extend(b"data");
        chunks.extend(self.data.to_bytes());
        
        // Header
        let mut result = Vec::with_capacity(chunks.len() + 12);
        result.extend(b"RIFF");
        result.extend((chunks.len() as u32 + 4).to_le_bytes());
        result.extend(b"WAVE");
        result.extend(std::mem::take(&mut chunks));
        
        result
    }
}
impl<T: WavFactExt> LgWavRaw<T> {
    pub fn decode(self) -> Result<LgWavDecoded<T>, String> {
        LgWavDecoded::try_from(self)
    }
    
    pub fn to_bytes(self) -> Vec<u8> {
        self.into()
    }
}