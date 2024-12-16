use std::io;
use crate::{tools, Result};

pub trait LgWriter {
    fn go_to(&mut self, position: usize) -> Result<usize>;

    fn write_le_u8(&mut self, data: u8) -> Result<()>;

    fn write_le_u16(&mut self, data: u16) -> Result<()>;
    
    fn write_le_u32(&mut self, data: u32) -> Result<()>;

    fn write_le_i8(&mut self, data: i8) -> Result<()>;
    
    fn write_le_i16(&mut self, data: i16) -> Result<()>;
    
    fn write_le_i32(&mut self, data: i32) -> Result<()>;

    fn write_le_i32_24(&mut self, data: i32) -> Result<()>;
    
    fn write_le_f32(&mut self, data: f32) -> Result<()>;
    
    fn write_le_f64(&mut self, data: f64) -> Result<()>;
}

impl<W: io::Write + io::Seek> LgWriter for W {
    fn go_to(&mut self, position: usize) -> Result<usize> {
        Ok(self.seek(io::SeekFrom::Start(position as u64))? as usize)
    }

    fn write_le_u8(&mut self, data: u8) -> Result<()> {
        write_bytes(self, &[data])
    }

    fn write_le_u16(&mut self, data: u16) -> Result<()> {
        write_bytes(self, &data.to_le_bytes())
    }

    fn write_le_u32(&mut self, data: u32) -> Result<()> {
        write_bytes(self, &data.to_le_bytes())
    }

    fn write_le_i8(&mut self, data: i8) -> Result<()> {
        let data = tools::i8_to_u8(data);
        
        self.write_le_u8(data)
    }

    fn write_le_i16(&mut self, data: i16) -> Result<()> {
        write_bytes(self, &data.to_le_bytes())
    }

    fn write_le_i32(&mut self, data: i32) -> Result<()> {
        write_bytes(self, &data.to_le_bytes())
    }

    fn write_le_i32_24(&mut self, data: i32) -> Result<()> {
        let clamped = data.clamp(-8_388_608, 8_388_607); // Clamp to 24-bit range
        let bytes = [
            (clamped & 0xFF) as u8,
            ((clamped >> 8) & 0xFF) as u8,
            ((clamped >> 16) & 0xFF) as u8,
        ];

        write_bytes(self, &bytes)
    }

    fn write_le_f32(&mut self, data: f32) -> Result<()> {
        write_bytes(self, &data.to_le_bytes())
    }

    fn write_le_f64(&mut self, data: f64) -> Result<()> {
        write_bytes(self, &data.to_le_bytes())
    }
}
pub fn write_bytes<W: io::Write + io::Seek>(writer: &mut W, bytes: &[u8]) -> Result<()> {
    writer.write(&bytes)?;
    
    Ok(())
}