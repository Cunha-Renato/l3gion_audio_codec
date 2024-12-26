use std::{fmt, fs, io, path};
use crate::{decoder::LgDecoder, error::Error, Result, SampleType};
use super::{reader::LgWavReader, LgWavSampleIter, WavChunks, WavFmt};

pub struct LgWavDecoder<R: io::Read> {
    fmt: WavFmt,
    sample_len: usize,

    reader: LgWavReader<R>,
}
impl<R: io::Read> fmt::Debug for LgWavDecoder<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LgWavDecoder")
            .field("fmt", &self.fmt)
            .field("sample_len", &self.sample_len)
            .finish()
    }
}
impl LgWavDecoder<io::BufReader<fs::File>> {
    pub fn new(path: impl AsRef<path::Path>) -> Result<Self> {
        let file = fs::File::open(path)?;
        // Already checks the header.
        let mut reader = LgWavReader::new(io::BufReader::new(file))?;
        
        // Just in case the fmt chunk is not present.
        let mut fmt = Err(Error::WrongFmt);
        let sample_len;

        loop { 
            let chunk = reader.read_next_chunk();
            match chunk? {
                WavChunks::FMT(wav_fmt) => fmt = Ok(wav_fmt),
                WavChunks::FACT => (),
                WavChunks::DATA(d_len) => {
                    match &mut fmt {
                        Ok(fmt) => sample_len = (d_len / (fmt.bits_per_sample as u32 / 8)) as usize,
                        Err(_) => return Err(Error::WrongFmt),
                    }

                    break;
                },
            }
        } 
        
        Ok(Self {
            fmt: fmt?,
            sample_len,
            reader,
        })
    }
    
    pub fn format(&self) -> WavFmt {
        self.fmt
    }
}
impl<R: io::Read> LgDecoder for LgWavDecoder<R> {
    type Info = WavFmt;

    fn info(&self) -> Self::Info {
        self.fmt
    }

    fn samples<S: super::Sample>(&mut self) -> impl Iterator<Item = S> {
        let sample_type = match self.fmt.format {
            super::WavFmtTag::WAVE_FORMAT_IEEE_FLOAT => SampleType::FLOAT,
            _ => SampleType::INT
        };

        LgWavSampleIter::new(&mut self.reader, sample_type, self.fmt.bits_per_sample)
    }

    fn duration(&self) -> usize {
        self.sample_len / self.fmt.channels as usize / self.fmt.sample_rate as usize
    }

    fn len(&self) -> usize {
        self.sample_len
    }
}