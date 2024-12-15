use crate::{error::Error, reader::LgReader, writer::LgWriter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SampleType {
    INT,
    FLOAT,
}

pub trait Sample: Sized {
    fn read(reader: &mut impl LgReader, sample_type: SampleType, bits_per_sample: u16) -> Result<Self>;
    
    fn write(self, writer: &mut impl LgWriter, sample_type: SampleType, bits_per_sample: u16) -> Result<()>;
}

impl Sample for i32 {
    fn read(reader: &mut impl LgReader, sample_type: SampleType, bits_per_sample: u16) -> Result<Self> {
        Ok(match (sample_type, bits_per_sample) {
            (SampleType::INT, 8) => reader.read_le_i8()? as i32,
            (SampleType::INT, 16) => reader.read_le_i16()? as i32,
            (SampleType::INT, 24) => reader.read_le_i32_24()?,
            (SampleType::INT, 32) => reader.read_le_i32()?,
            (SampleType::FLOAT, 32) => reader.read_le_f32()? as i32,
            (SampleType::FLOAT, 64) => reader.read_le_f64()? as i32,

            _ => return Err(Error::Conversion(std::format!("{:?} with {} bits per sample is not supported for i32!", sample_type, bits_per_sample))),
        })
    }
    
    fn write(self, writer: &mut impl LgWriter, sample_type: SampleType, bits_per_sample: u16) -> Result<()> {
        match (sample_type, bits_per_sample) {
            (SampleType::INT, 8) => writer.write_le_i8(self as i8)?,
            (SampleType::INT, 16) => writer.write_le_i16(self as i16)?,
            (SampleType::INT, 24) => writer.write_le_i32_24(self)?,
            (SampleType::INT, 32) => writer.write_le_i32(self)?,
            (SampleType::FLOAT, 32) => writer.write_le_f32(self as f32)?,
            (SampleType::FLOAT, 64) => writer.write_le_f64(self as f64)?,

            _ => return Err(Error::Conversion(std::format!("{:?} with {} bits per sample is not supported for i32!", sample_type, bits_per_sample))),           
        }
        
        
        Ok(())
    }
}