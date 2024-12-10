use std::result;

pub mod reader;
pub mod error;
pub mod tools;

pub mod wav;

pub type Result<T> = result::Result<T, error::Error>;