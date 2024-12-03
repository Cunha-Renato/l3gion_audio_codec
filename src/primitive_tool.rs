pub(crate) trait FromLeBytesSlice {
    fn first_from_le_bytes(bytes: &[u8]) -> Self;
}
impl FromLeBytesSlice for u16 {
    fn first_from_le_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2);
        Self::from_le_bytes([bytes[0], bytes[1]])
    }
}
impl FromLeBytesSlice for u32 {
    fn first_from_le_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 4);
        
        Self::from_le_bytes([
            bytes[0], 
            bytes[1],
            bytes[2],
            bytes[3],
        ])
    }
}
impl FromLeBytesSlice for u128 {
    fn first_from_le_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 4);
        Self::from_le_bytes([
            bytes[0], 
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4], 
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8], 
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12], 
            bytes[13],
            bytes[14],
            bytes[15],
        ])
    }
}