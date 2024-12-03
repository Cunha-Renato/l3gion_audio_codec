use super::chunk::LgWavChunks;

#[derive(Debug, Default, Clone)]
pub struct LgWavProcessed {

}
impl<T> From<LgWavChunks<T>> for LgWavProcessed 
where
    T: TryFrom<Vec<u8>>,
    T: Into<Vec<u8>>,
    T: std::fmt::Debug,
    T: Clone,
{
    fn from(chunks: LgWavChunks<T>) -> Self {
        
        
        todo!()
    }
}