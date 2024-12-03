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
impl From<Vec<u8>> for WavDataChunk {
    fn from(mut bytes: Vec<u8>) -> Self {
        let ck_size = bytes.len();
        
        // If is odd
        if ck_size % 2 != 0 {
            // Remove padding byte.
            bytes.pop();
        }
        
        Self {
            ck_size,
            data: bytes,
        }
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