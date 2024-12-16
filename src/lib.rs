use std::result;

pub mod info;
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