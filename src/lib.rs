use std::result;

pub mod decoder;
pub mod encoder;
pub mod reader;
pub mod writer;
pub mod error;
pub mod tools;
pub mod wav;
pub mod sample;
pub use sample::*;

pub type Result<T> = result::Result<T, error::Error>;

#[derive(Default, Debug, Clone, Copy)]
pub struct AudioInfo {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub sample_type: Option<SampleType>,
}