use std::{io, ops::{Deref, DerefMut}};
use crate::{tools::u8_to_i8, Result};

pub trait LgReader {
    fn read_into(&mut self, buffer: &mut [u8]) -> Result<()>;

    fn read_next_bytes<const N: usize>(&mut self) -> Result<[u8; N]>;

    fn skip_next_bytes<const N: usize>(&mut self) -> Result<()>;

    fn read_le_u8(&mut self) -> Result<u8>;

    fn read_le_u16(&mut self) -> Result<u16>;
    
    fn read_le_u32(&mut self) -> Result<u32>;

    fn read_le_i8(&mut self) -> Result<i8>;
    
    fn read_le_i16(&mut self) -> Result<i16>;
    
    fn read_le_i32(&mut self) -> Result<i32>;

    fn read_le_i32_24(&mut self) -> Result<i32>;
    
    fn read_le_f32(&mut self) -> Result<f32>;
    
    fn read_le_f64(&mut self) -> Result<f64>;
}

pub struct LgFileReader<R: io::Read>(pub(crate) R);
impl<R: io::Read> Deref for LgFileReader<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<R: io::Read> DerefMut for LgFileReader<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<R: io::Read> io::Read for LgFileReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
impl<R: io::Read> LgReader for LgFileReader<R> {
    fn read_into(&mut self, buffer: &mut [u8]) -> Result<()> {
        self.read_exact(buffer)?;

        Ok(())
    }

    fn read_next_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;

        Ok(buf)
    }

    fn skip_next_bytes<const N: usize>(&mut self) -> Result<()> {
        self.read_exact(&mut [0; N])?;

        Ok(())
    }

    fn read_le_u8(&mut self) -> Result<u8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        
        Ok(buf[0])
    }

    fn read_le_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        
        Ok(u16::from_le_bytes(buf))
    }
    
    fn read_le_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        
        Ok(u32::from_le_bytes(buf))
    }

    fn read_le_i8(&mut self) -> Result<i8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        
        Ok(u8_to_i8(buf[0]))
    }
    
    fn read_le_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        
        Ok(i16::from_le_bytes(buf))
    }
    
    fn read_le_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        
        Ok(i32::from_le_bytes(buf))
    }

    fn read_le_i32_24(&mut self) -> Result<i32> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf)?;

        let i32_bytes = [
            buf[0],
            buf[1],
            buf[2],
            if buf[2] & 0x80 != 0 { 0xFF } else { 0x00 }
        ];
        
        Ok(i32::from_le_bytes(i32_bytes))
    }
    
    fn read_le_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        
        Ok(f32::from_le_bytes(buf))
    }
    
    fn read_le_f64(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        
        Ok(f64::from_le_bytes(buf))
    }
}