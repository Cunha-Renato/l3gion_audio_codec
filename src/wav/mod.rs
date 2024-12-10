use std::fs::File;
use std::marker::PhantomData;
use std::{ffi::OsStr, fmt::Debug, io};

use chunk::WavFmt;
use reader::LgWavReader;

use crate::error::Error;
use crate::reader::{LgFileReader, LgReader};
use crate::Result;

pub mod reader;
pub mod chunk;

const WAVE_FORMAT_PCM: u16 =        0x0001;
const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;
const WAVE_FORMAT_ALAW: u16 =       0x0006;
const WAVE_FORMAT_MULAW: u16 =      0x0007;
const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum WavFmtTag {
    #[default]
    WAVE_FORMAT_PCM,
    WAVE_FORMAT_IEEE_FLOAT,
    WAVE_FORMAT_ALAW,
    WAVE_FORMAT_MULAW,
    WAVE_FORMAT_EXTENSIBLE,
    OTHER(u16)
}
impl From<u16> for WavFmtTag {
    fn from(value: u16) -> Self {
        match value {
            WAVE_FORMAT_PCM =>          Self::WAVE_FORMAT_PCM,
            WAVE_FORMAT_IEEE_FLOAT =>   Self::WAVE_FORMAT_IEEE_FLOAT,
            WAVE_FORMAT_ALAW =>         Self::WAVE_FORMAT_ALAW,
            WAVE_FORMAT_MULAW =>        Self::WAVE_FORMAT_MULAW,
            WAVE_FORMAT_EXTENSIBLE =>   Self::WAVE_FORMAT_EXTENSIBLE,
            _ => Self::OTHER(value),
        }
    }
}
impl Into<u16> for WavFmtTag {
    fn into(self) -> u16 {
        match self {
            Self::WAVE_FORMAT_PCM =>        WAVE_FORMAT_PCM,
            Self::WAVE_FORMAT_IEEE_FLOAT => WAVE_FORMAT_IEEE_FLOAT,
            Self::WAVE_FORMAT_ALAW =>       WAVE_FORMAT_ALAW,
            Self::WAVE_FORMAT_MULAW =>      WAVE_FORMAT_MULAW,
            Self::WAVE_FORMAT_EXTENSIBLE => WAVE_FORMAT_EXTENSIBLE,
            Self::OTHER(value) => value,
        }
    }
}

pub struct LgWav<R: io::Read> {
    pub fmt: WavFmt,
    sample_len: usize,

    reader: LgWavReader<R>,
}
impl<R: io::Read> Debug for LgWav<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LgWav")
        .field("fmt", &self.fmt)
        .finish()
    }
}
impl LgWav<io::BufReader<File>> {
    pub fn decode(path: impl AsRef<OsStr>) -> Result<LgWav<io::BufReader<File>>> {
        let file = File::open(std::path::Path::new(&path))?;
        // Already checks the header.
        let mut reader = LgWavReader::new(LgFileReader(io::BufReader::new(file)))?;
        
        // Just in case the fmt chunk is not present.
        let mut fmt = Err(Error::WrongFmt);
        let sample_len;

        loop { 
            let chunk = reader.read_next_chunk();
            match chunk? {
                chunk::WavChunks::FMT(wav_fmt) => fmt = Ok(wav_fmt),
                chunk::WavChunks::FACT => (),
                chunk::WavChunks::DATA(d_len) => {
                    match &mut fmt {
                        Ok(fmt) => sample_len = (d_len / (fmt.bits_per_sample as u32 / 8)) as usize,
                        Err(_) => return Err(Error::WrongFmt),
                    }

                    break;
                },
            }
        } 
        
        Ok(LgWav {
            fmt: fmt?,
            sample_len,
            reader,
        })
    }
    
    /// Length of the samples.
    pub const fn len(&self) -> usize {
        self.sample_len
    }
    
    /// Duration of the audio in seconds.
    pub const fn duration(&self) -> usize {
        self.sample_len / self.fmt.channels as usize / self.fmt.sample_rate as usize
    }

    /// Iterator over the samples.
    /// Once you iterate over the elements, calling this again will not be on the start of 
    /// the samples, so it is recommended that you store the samples in a container if 
    /// you need to reuse them.
    pub fn samples<S: Sample<io::BufReader<File>>>(&mut self) -> LgWavSampleIter<io::BufReader<File>, S> {
        LgWavSampleIter::new(&mut self.reader, &self.fmt)
    }
}

pub struct LgWavSampleIter<'si, R: io::Read, S: Sample<R>> {
    fmt: &'si WavFmt,
    reader: &'si mut LgWavReader<R>,
    _phantom: PhantomData<S>,
}
impl<'si, R: io::Read, S: Sample<R>> LgWavSampleIter<'si, R, S> {
    fn new(reader: &'si mut LgWavReader<R>, fmt: &'si WavFmt) -> Self {
        Self {
            fmt,
            reader,
            _phantom: PhantomData,
        }
    }
}
impl<'si, R: io::Read, S: Sample<R>> Iterator for LgWavSampleIter<'si, R, S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        S::read(self.reader, self.fmt).ok()
    }
}

pub trait Sample<R: io::Read>: Sized {
    fn read(reader: &mut LgWavReader<R>, fmt: &WavFmt) -> Result<Self>;
}

impl<R: io::Read> Sample<R> for i32 {
    fn read(reader: &mut LgWavReader<R>, fmt: &WavFmt) -> Result<Self> {
        Ok(match (fmt.format, fmt.bits_per_sample) {
            (WavFmtTag::WAVE_FORMAT_PCM, 8) => reader.read_le_i8()? as i32,
            (WavFmtTag::WAVE_FORMAT_PCM, 16) => reader.read_le_i16()? as i32,
            (WavFmtTag::WAVE_FORMAT_PCM, 24) => reader.read_le_i32_24()?,
            (WavFmtTag::WAVE_FORMAT_PCM, 32) => reader.read_le_i32()?,
            (WavFmtTag::WAVE_FORMAT_IEEE_FLOAT, 32) => reader.read_le_f32()? as i32,
            (WavFmtTag::WAVE_FORMAT_IEEE_FLOAT, 64) => reader.read_le_f64()? as i32,

            _ => return Err(Error::Conversion(std::format!("{:?} with {} bits per sample is not supported for i32!", fmt.format, fmt.bits_per_sample))),
        })
    }
}