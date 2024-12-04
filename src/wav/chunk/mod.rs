use data::WavDataChunk;
use fact::{WavFactChunk, WavFactExt};
use fmt::WavFmtChunk;

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