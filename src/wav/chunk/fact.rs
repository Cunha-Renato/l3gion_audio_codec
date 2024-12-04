use std::fmt::Debug;

use crate::{parser::error::LgAudioParseErr, primitive_tool::FromLeBytesSlice, reader::LgVecReader};

pub trait WavFactExt: Debug + Default + Clone + Into<Vec<u8>> + for<'a> From<&'a [u8]> {}

#[derive(Debug, Clone)]
pub struct WavFactChunk<T> 
where 
    T: WavFactExt
{
    pub ck_size: usize,
    pub sample_length: u32,
    pub other: T,
}
impl<T> WavFactChunk<T> 
where 
    T: WavFactExt,
{
    pub fn read_bytes(ck_size: usize, bytes: &mut LgVecReader<u8>) -> Result<Self, LgAudioParseErr> {
        let (sample_length, other) = if ck_size > 4 {
            (
                u32::first_from_le_bytes(bytes.read_quantity(4)?),
                T::from(bytes.read_quantity(ck_size - 4)?)
            )

        }
        else {
            (
                u32::first_from_le_bytes(bytes.read_quantity(4)?),
                T::default(),
            )
        };

        Ok(Self {
            ck_size,
            sample_length,
            other,
        })
    }
}