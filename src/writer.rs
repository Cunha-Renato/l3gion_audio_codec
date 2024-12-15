use std::{io, ops::{Deref, DerefMut}};
use crate::{tools, Result};

pub trait LgWriter {
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

pub struct LgFileWriter<W: io::Write>(pub(crate) W);
impl<W: io::Write> LgFileWriter<W> {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.write(&bytes)?;
        
        Ok(())
    }
}
impl<W: io::Write> Deref for LgFileWriter<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<W: io::Write> DerefMut for LgFileWriter<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<W: io::Write> io::Write for LgFileWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
impl<W: io::Write> LgWriter for LgFileWriter<W> {
    fn write_le_u8(&mut self, data: u8) -> Result<()> {
        self.write_bytes(&[data])
    }

    fn write_le_u16(&mut self, data: u16) -> Result<()> {
        self.write_bytes(&data.to_le_bytes())
    }

    fn write_le_u32(&mut self, data: u32) -> Result<()> {
        self.write_bytes(&data.to_le_bytes())
    }

    fn write_le_i8(&mut self, data: i8) -> Result<()> {
        let data = tools::i8_to_u8(data);
        
        self.write_le_u8(data)
    }

    fn write_le_i16(&mut self, data: i16) -> Result<()> {
        self.write_bytes(&data.to_le_bytes())
    }

    fn write_le_i32(&mut self, data: i32) -> Result<()> {
        self.write_bytes(&data.to_le_bytes())
    }

    fn write_le_i32_24(&mut self, data: i32) -> Result<()> {
        let bytes = data.to_le_bytes();

        self.write_bytes(&[bytes[0], bytes[1], bytes[2]])
    }

    fn write_le_f32(&mut self, data: f32) -> Result<()> {
        self.write_bytes(&data.to_le_bytes())
    }

    fn write_le_f64(&mut self, data: f64) -> Result<()> {
        self.write_bytes(&data.to_le_bytes())
    }
}