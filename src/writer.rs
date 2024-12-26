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
        self.write(&[data])?;

        Ok(())
    }

    fn write_le_u16(&mut self, data: u16) -> Result<()> {
        self.write(&data.to_le_bytes())?;

        Ok(())
    }

    fn write_le_u32(&mut self, data: u32) -> Result<()> {
        self.write(&data.to_le_bytes())?;

        Ok(())
    }

    fn write_le_i8(&mut self, data: i8) -> Result<()> {
        let data = tools::i8_to_u8(data);
        
        self.write_le_u8(data)
    }

    fn write_le_i16(&mut self, data: i16) -> Result<()> {
        self.write(&data.to_le_bytes())?;

        Ok(())
    }

    fn write_le_i32(&mut self, data: i32) -> Result<()> {
        self.write(&data.to_le_bytes())?;

        Ok(())
    }

    fn write_le_i32_24(&mut self, data: i32) -> Result<()> {
        let buf = data.to_le_bytes();
        self.write_all(&[buf[0], buf[1], buf[2]])?;
        
        Ok(())
    }

    fn write_le_f32(&mut self, data: f32) -> Result<()> {
        self.write_all(&data.to_le_bytes())?;
        
        Ok(())
    }

    fn write_le_f64(&mut self, data: f64) -> Result<()> {
        self.write_all(&data.to_le_bytes())?;
        
        Ok(())
    }
}