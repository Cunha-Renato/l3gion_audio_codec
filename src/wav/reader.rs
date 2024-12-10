use std::{io, usize};
use crate::error::Error;
use crate::reader::{LgFileReader, LgReader};
use crate::wav::WavFmtTag;
use crate::Result;

use super::chunk::{WavChunks, WavFmt};
use super::Sample;

pub struct LgWavReader<R: io::Read> {
    reader: LgFileReader<R>,
    max_size: usize,
    cursor: usize,
}
impl<R: io::Read> LgReader for LgWavReader<R> {
    fn read_next_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        self.move_cursor(N)?;

        let mut buf = [0; N];
        self.reader.read_exact(&mut buf)?;

        Ok(buf)
    }

    fn skip_next_bytes<const N: usize>(&mut self) -> Result<()> {
        self.move_cursor(N)?;

        self.reader.read_exact(&mut [0; N])?;
        
        Ok(())
    }

    fn read_le_u8(&mut self) -> Result<u8> {
        self.move_cursor(1)?;

        let mut buf = [0];
        self.reader.read_exact(&mut buf)?;
    
        Ok(buf[0])
    }

    fn read_le_u16(&mut self) -> Result<u16> {
        self.move_cursor(2)?;

        let mut buf = [0; 2];
        self.reader.read_exact(&mut buf)?;
        
        Ok(u16::from_le_bytes(buf))
    }

    fn read_le_u32(&mut self) -> Result<u32> {
        self.move_cursor(4)?;

        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
    
        Ok(u32::from_le_bytes(buf))
    }

    fn read_le_i8(&mut self) -> Result<i8> {
        self.move_cursor(1)?;

        let mut buf = [0];
        self.reader.read_exact(&mut buf)?;
        
        Ok(crate::tools::u8_to_i8(buf[0]))
    }

    fn read_le_i16(&mut self) -> Result<i16> {
        self.move_cursor(2)?;

        let mut buf = [0; 2];
        self.reader.read_exact(&mut buf)?;
    
        Ok(i16::from_le_bytes(buf))
    }

    fn read_le_i32(&mut self) -> Result<i32> {
        self.move_cursor(4)?;

        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
    
        Ok(i32::from_le_bytes(buf))
    }

    fn read_le_i32_24(&mut self) -> Result<i32> {
        self.move_cursor(3)?;

        let mut buf = [0; 3];
        self.reader.read_exact(&mut buf)?;

        let i32_bytes = [
            buf[0],
            buf[1],
            buf[2],
            if buf[2] & 0x80 != 0 { 0xFF } else { 0x00 }
        ];
    
        Ok(i32::from_le_bytes(i32_bytes))
    }

    fn read_le_f32(&mut self) -> Result<f32> {
        self.move_cursor(4)?;

        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
    
        Ok(f32::from_le_bytes(buf))
    }

    fn read_le_f64(&mut self) -> Result<f64> {
        self.move_cursor(8)?;

        let mut buf = [0; 8];
        self.reader.read_exact(&mut buf)?;
    
        Ok(f64::from_le_bytes(buf))
    }
}
impl<R: io::Read> LgWavReader<R> {
    pub(super) fn new(reader: LgFileReader<R>) -> Result<Self> {
        Self::read_header(reader)    
    }

    pub(super) fn read_header(mut reader: LgFileReader<R>) -> Result<Self> {
        if b"RIFF" != &reader.read_next_bytes()? {
            return Err(Error::WrongHeader);
        }
        
        let ck_size = reader.read_le_u32()? - 4;
        
        if b"WAVE" != &reader.read_next_bytes()? {
            return Err(Error::WrongHeader);
        }
        
        Ok(Self {
            reader,
            max_size: ck_size as usize,
            cursor: 0,
        })
    }

    pub(super) fn read_next_chunk(&mut self) -> Result<WavChunks> {
        Ok(match &self.read_next_bytes()? {
            b"fmt " => WavChunks::FMT(self.read_fmt_chunk()?),
            b"data" => WavChunks::DATA(self.read_le_u32()?),

            _ => return Err(Error::WrongFmtInfo("Currently do not support more chunks other than fmt and data!".to_string())),
        })
    }
    
    pub(super) fn read_fmt_chunk(&mut self) -> Result<WavFmt> {
        let ck_size = self.read_le_u32()? as usize;

        if ck_size > 40 || ck_size < 16 { return Err(Error::WrongFmt); }

        let fmt_tag: WavFmtTag = self.read_le_u16()?.into();
        let channels = self.read_le_u16()?;
        let samples_per_sec = self.read_le_u32()?;
        let _avg_bytes_per_sec = self.read_le_u32()?;
        let _block_align = self.read_le_u16()?;
        let bits_per_sample = self.read_le_u16()?;

        let mut fmt = WavFmt {
            format: fmt_tag,
            channels,
            sample_rate: samples_per_sec,
            bits_per_sample,
        };

        // Time to check if the info is ok.
        check_fmt(&fmt)?;

        match (fmt_tag, ck_size) {
            (WavFmtTag::WAVE_FORMAT_PCM, ck_size) => self.read_check_fmt_pcm(ck_size, &fmt)?,
            (WavFmtTag::WAVE_FORMAT_IEEE_FLOAT, ck_size) => self.read_check_fmt_ieee_float(ck_size, &fmt)?,
            (WavFmtTag::WAVE_FORMAT_EXTENSIBLE, ck_size) => self.read_check_fmt_extensible(ck_size, &mut fmt)?,
            // TODO: ALAW.
            // TODO: MULAW.

            _ => return Err(Error::WrongFmt),
        };
        //
        // 4 bytes for the ck_id.
        // 4 bytes for the ck_size.
        let bytes_to_skip = (ck_size + 8) - self.cursor;
        self.cursor += bytes_to_skip;

        // 4 bytes for the ck_id.
        // 4 bytes for the ck_size.
        assert_eq!(self.cursor, 8 + ck_size);

        Ok(fmt)
    }
    
    fn read_check_fmt_pcm(&mut self, ck_size :usize, fmt: &WavFmt) -> Result<()> {
        // If ck_size is 16, that means that all the fmt was read.
        if ck_size == 16 { return Ok(()); }

        // If this executes then it means that is a WAVEFORMATEX.
        
        // Dealing with cb_size.
        self.skip_next_bytes::<2>()?;
        
        // Dealing with bits_per_sample.
        if fmt.bits_per_sample > 24 || fmt.bits_per_sample < 8 { 
            return Err(Error::WrongFmtInfo("Invalid bits_per_sample for PCM format!".to_string()));
        }
        
        Ok(())
    }

    fn read_check_fmt_ieee_float(&mut self, ck_size :usize, fmt: &WavFmt) -> Result<()> {
        // If ck_size is 16, that means that all the fmt was read.
        if ck_size == 16 { return Ok(()); }
        if ck_size != 18 { 
            return Err(Error::WrongFmtInfo("IEEE_FLOAT does not alow for ck_size > 18!".to_string())); 
        }

        // Dealing with cb_size.
        if self.read_le_u16()? != 0 {
            return Err(Error::WrongFmtInfo("IEEE_FLOAT must have cb_size of 0!".to_string()));
        }

        Ok(())
    }
    
    fn read_check_fmt_extensible(&mut self, ck_size :usize, fmt: &mut WavFmt) -> Result<()> {
        if ck_size < 40 {
            return Err(Error::WrongFmtInfo("WAVE_FORMAT_EXTENSIBLE must have ck_size of 40!".to_string()));
        }
        
        // Dealing with cb_size.
        if self.read_le_u16()? != 22 {
            return Err(Error::WrongFmtInfo("WAVE_FORMAT_EXTENSIBLE must have cb_size of 22!".to_string()));
        }
        
        let valid_bits_per_sample = self.read_le_u16()?;
        // Skip channel_mask.
        self.skip_next_bytes::<4>()?;
        // GUID
        let sub_format: [u8; 16] = self.read_next_bytes()?;

        // TODO: Support different GUIDs.

        if valid_bits_per_sample > 0 {
            fmt.bits_per_sample = valid_bits_per_sample;
        }

        Ok(())
    }
}
impl<R: io::Read> LgWavReader<R> {
    fn move_cursor(&mut self, n: usize) -> Result<()> {
        if self.cursor + n > self.max_size + 1 {
            return Err(Error::Io(io::Error::new::<String>(io::ErrorKind::UnexpectedEof, "".into())));
        }
        
        self.cursor += n;

        Ok(())
    }
    
    fn check_cusor(&self) -> bool {
        self.cursor > self.max_size + 1
    }
}

fn check_fmt(fmt: &WavFmt) -> Result<()> {
    if fmt.channels == 0 {
        return Err(Error::WrongFmtInfo("fmt.channels must be > 0!".to_string()));
    }
    
    if fmt.bits_per_sample % 8 != 0 || fmt.bits_per_sample == 0 {
        return Err(Error::WrongFmtInfo("bits_per_sample must be non 0 and a multiple of 8!".to_string()));
    }

    Ok(())
}