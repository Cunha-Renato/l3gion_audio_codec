use crate::{error::Error, reader::LgReader, tools, writer::LgWriter, Result};

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
            (SampleType::FLOAT, 32) => tools::f32_to_i32(reader.read_le_f32()?),
            (SampleType::FLOAT, 64) => tools::f64_to_i32(reader.read_le_f64()?),

            _ => return Err(Error::Conversion(std::format!("{:?} with {} bits per sample is not supported for i32!", sample_type, bits_per_sample))),
        })
    }
    
    fn write(self, writer: &mut impl LgWriter, sample_type: SampleType, bits_per_sample: u16) -> Result<()> {
        match (sample_type, bits_per_sample) {
            (SampleType::INT, 8) => writer.write_le_i8(self as i8),
            (SampleType::INT, 16) => writer.write_le_i16(self as i16),
            (SampleType::INT, 24) => writer.write_le_i32_24(self),
            (SampleType::INT, 32) => writer.write_le_i32(self),
            (SampleType::FLOAT, 32) => writer.write_le_f32(tools::i32_to_f32(self)),
            (SampleType::FLOAT, 64) => writer.write_le_f64(tools::i32_to_f64(self)),

            _ => return Err(Error::Conversion(std::format!("{:?} with {} bits per sample is not supported for i32!", sample_type, bits_per_sample))),           
        }
    }
}

impl Sample for f32 {
    fn read(reader: &mut impl LgReader, sample_type: SampleType, bits_per_sample: u16) -> Result<Self> {
        let int_value = match (sample_type, bits_per_sample) {
            (SampleType::INT, 8) => reader.read_le_i8()? as i32,
            (SampleType::INT, 16) => reader.read_le_i16()? as i32,
            (SampleType::INT, 24) => reader.read_le_i32_24()?,
            (SampleType::INT, 32) => reader.read_le_i32()?,
            
            (SampleType::FLOAT, 32) => return Ok(reader.read_le_f32()?),
            (SampleType::FLOAT, 64) => return Ok(reader.read_le_f64()? as f32),

            _ => return Err(Error::Conversion(std::format!("{:?} with {} bits per sample is not supported for i32!", sample_type, bits_per_sample))),
        };
        
        Ok(tools::i32_to_f32(int_value))
    }
    
    fn write(self, writer: &mut impl LgWriter, sample_type: SampleType, bits_per_sample: u16) -> Result<()> {
        let int_value = tools::f32_to_i32(self);

        match (sample_type, bits_per_sample) {
            (SampleType::INT, 8) => writer.write_le_i8(int_value as i8),
            (SampleType::INT, 16) => writer.write_le_i16(int_value as i16),
            (SampleType::INT, 24) => writer.write_le_i32_24(int_value),
            (SampleType::INT, 32) => writer.write_le_i32(int_value),
            (SampleType::FLOAT, 32) => writer.write_le_f32(self),
            (SampleType::FLOAT, 64) => writer.write_le_f64(self as f64),

            _ => return Err(Error::Conversion(std::format!("{:?} with {} bits per sample is not supported for i32!", sample_type, bits_per_sample))),           
        }
    }
}