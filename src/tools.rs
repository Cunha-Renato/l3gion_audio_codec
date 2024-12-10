#[inline(always)]
pub fn u8_to_i8(value: u8) -> i8 {
    (value as i16 - 128) as i8
}

#[inline(always)]
pub fn i8_to_u8(value: i8) -> u8 {
    (value as i16 + 128) as u8
}