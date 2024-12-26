use crate::{Sample, Result};

pub trait LgEncoder {
    type Info;

    fn info(&self) -> Self::Info;
    
    fn encode_sample<S: Sample>(&mut self, sample: S) -> Result<()>;
    
    /// Number of samples encoded so far.
    fn encoded_samples(&self) -> usize;
}