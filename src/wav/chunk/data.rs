use crate::reader::LgVecReader;

#[derive(Default, Clone)]
pub struct WavDataChunk {
    ck_size: usize,
    data: Vec<u8>,
}
impl std::fmt::Display for WavDataChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WavDataChunk")
            .field("ck_size", &self.ck_size)
            .field("data_size", &self.data.len())
            .finish()
    }
}
impl std::fmt::Debug for WavDataChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl Into<Vec<u8>> for WavDataChunk {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.ck_size);
        result.extend(self.ck_size.to_le_bytes());
        result.extend(self.data);

        // Padding byte.
        if self.ck_size % 2 != 0 {
            result.push(0);
        }

        result
    }
}
impl WavDataChunk {
    pub fn read_bytes(ck_size: usize, bytes: &mut LgVecReader<u8>) -> Self {
        let mut data = bytes.read_quantity(ck_size).to_vec();
        
        // If is odd
        if ck_size % 2 != 0 {
            // Remove padding byte.
            data.pop();
        }
        
        Self {
            ck_size,
            data,
        }
    }
}