pub trait LeByteConversion {
    fn to_lbytes(&self) -> Vec<u8>;

    fn try_from_lbytes(bytes: &[u8]) -> Option<Self> where Self: Sized;
}
impl LeByteConversion for u8 {
    fn to_lbytes(&self) -> Vec<u8> {
        vec![*self]
    }

    fn try_from_lbytes(bytes: &[u8]) -> Option<Self> where Self: Sized {
        if bytes.is_empty() { return None; }
        Some(bytes[0])
    }
}

#[macro_export]
macro_rules! impl_le_byte_conversion {
    (2, $($t:ty),*) => {
        $(
            impl LeByteConversion for $t {
                fn to_lbytes(&self) -> Vec<u8> {
                    self.to_le_bytes().to_vec()
                }
                
                fn try_from_lbytes(bytes: &[u8]) -> Option<Self> where Self: Sized {
                    if bytes.len() != 2 { return None; }
                    Some(Self::from_le_bytes([bytes[0], bytes[1]]))
                }
            }
        )*
    };

    (4, $($t:ty),*) => {
        $(
            impl LeByteConversion for $t {
                fn to_lbytes(&self) -> Vec<u8> {
                    self.to_le_bytes().to_vec()
                }
                
                fn try_from_lbytes(bytes: &[u8]) -> Option<Self> where Self: Sized {
                    if bytes.len() != 4 { return None; }
                    Some(Self::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
                }
            }
        )*
    };

    (8, $($t:ty),*) => {
        $(
            impl LeByteConversion for $t {
                fn to_lbytes(&self) -> Vec<u8> {
                    self.to_le_bytes().to_vec()
                }
                
                fn try_from_lbytes(bytes: &[u8]) -> Option<Self> where Self: Sized {
                    if bytes.len() != 8 { return None; }
                    Some(Self::from_le_bytes([
                        bytes[0], 
                        bytes[1], 
                        bytes[2], 
                        bytes[3],
                        bytes[4], 
                        bytes[5], 
                        bytes[6], 
                        bytes[7]
                    ]))
                }
            }
        )*
    };
}

impl_le_byte_conversion!(2, u16, i16);
impl_le_byte_conversion!(4, u32, i32, f32);
impl_le_byte_conversion!(8, f64);