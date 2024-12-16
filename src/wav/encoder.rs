use std::{fs, io, path};

use crate::{encoder::LgEncoder, info::LgAudioInfo, Result, SampleType};
use super::{writer::LgWavWriter, WavFmt};

pub struct LgWavEncoder<W: io::Write + io::Seek> {
    pub(super) fmt: WavFmt,
    writer: LgWavWriter<W>,
}
impl LgWavEncoder<io::BufWriter<fs::File>> {
    pub fn new(path: impl AsRef<path::Path>, fmt: WavFmt) -> Result<Self> {
        let file = fs::File::create(path)?;
        let writer = LgWavWriter::new(io::BufWriter::new(file), &fmt)?;

        Ok(Self {
            fmt,
            writer,
        })
    }
    
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
    
    pub fn finish(&mut self) -> Result<()> {
        self.writer.finish()
    }
}
impl<W: io::Write + io::Seek>  LgEncoder for LgWavEncoder<W> {
    fn info(&self) -> LgAudioInfo {
        LgAudioInfo::WAV(self.fmt)
    }
    
    fn encode_sample<S: crate::Sample>(&mut self, sample: S) -> Result<()> {
        let sample_type = match self.fmt.format {
            super::WavFmtTag::WAVE_FORMAT_IEEE_FLOAT => SampleType::FLOAT,
            _ => SampleType::INT,
        };

        self.writer.write_sample(sample, sample_type, self.fmt.bits_per_sample)
    }
    
    fn encoded_samples(&self) -> usize {
        self.writer.data_bytes_written as usize * self.fmt.bits_per_sample as usize
    }

}