use data::WavDataChunk;
use fact::WavFactChunk;
use fmt::WavFmtChunk;

pub mod fmt;
pub mod fact;
pub mod data;

#[derive(Debug, Clone)]
pub struct LgWavChunks<T> 
where 
    T: TryFrom<Vec<u8>>,
    T: Into<Vec<u8>>,
    T: std::fmt::Debug,
    T: Clone,
{
    pub fmt: WavFmtChunk,
    pub fact: Option<WavFactChunk<T>>,
    pub data: WavDataChunk,
}
impl<T> std::default::Default for LgWavChunks<T>
where 
    T: TryFrom<Vec<u8>>,
    T: Into<Vec<u8>>,
    T: std::fmt::Debug,
    T: Clone,
{
    fn default() -> Self {
        Self { 
            fmt: Default::default(), 
            fact: Default::default(), 
            data: Default::default() 
        }
    }
}