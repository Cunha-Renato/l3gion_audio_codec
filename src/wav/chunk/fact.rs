use std::fmt::Debug;

use crate::primitive_tool::FromLeBytesSlice;

#[derive(Debug, Clone)]
pub struct WavFactChunk<T> 
where 
    T: TryFrom<Vec<u8>>,
    T: Into<Vec<u8>>,
    T: Debug,
    T: Clone,
{
    pub ck_size: usize,
    pub sample_length: u32,
    pub other: T,
}
impl<T> TryFrom<Vec<u8>> for WavFactChunk<T> 
where 
    T: TryFrom<Vec<u8>>,
    T: Into<Vec<u8>>,
    T: Debug,
    T: Clone,
    <T as TryFrom<Vec<u8>>>::Error: std::error::Error + 'static
{
    type Error = Box<dyn std::error::Error>;

    fn try_from(mut bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let ck_size = bytes.len();

        let sample_length = if ck_size > 4 {
            u32::first_from_le_bytes(&bytes.split_off(4))
        }
        else {
            u32::first_from_le_bytes(&std::mem::take(&mut bytes))
        };

        Ok(Self {
            ck_size,
            sample_length,
            other: T::try_from(bytes)?,
        })
    }
}