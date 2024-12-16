use crate::{info::LgAudioInfo, Sample, Result};

pub trait LgEncoder {
    fn info(&self) -> LgAudioInfo;
    
    fn encode_sample<S: Sample>(&mut self, sample: S) -> Result<()>;
    
    /// Number of samples encoded so far.
    fn encoded_samples(&self) -> usize;
}