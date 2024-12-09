use std::fmt::Debug;

use crate::{parser::error::LgAudioParseErr, primitive_tool::FromLeBytesSlice, reader::LgVecReader};

pub trait WavFactExt: Debug + Default + Clone + Into<Vec<u8>> + for<'a> From<&'a [u8]> {}
impl WavFactExt for Vec<u8> {}

#[derive(Debug, Clone)]
pub struct WavFactChunk<T: WavFactExt> {
    pub ck_size: usize,
    pub sample_length: u32,
    pub other: T,
}
impl<T: WavFactExt> Into<Vec<u8>> for WavFactChunk<T> {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.ck_size + 4);
        result.extend((self.ck_size as u32).to_le_bytes());
        result.extend(self.sample_length.to_le_bytes());
        result.extend(self.other.into());
        result.truncate(self.ck_size + 4);
        
        result
    }
}
impl<T: WavFactExt> WavFactChunk<T> {
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
    
    pub fn to_bytes(self) -> Vec<u8> {
        self.into()
    }
}