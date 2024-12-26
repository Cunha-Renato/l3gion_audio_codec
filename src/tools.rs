#[inline(always)]
pub fn u8_to_i8(value: u8) -> i8 {
    (value as i16 - 128) as i8
}

#[inline(always)]
pub fn i8_to_u8(value: i8) -> u8 {
    (value as i16 + 128) as u8
}

#[inline(always)]
pub fn f32_to_i32(value: f32) -> i32 {
    (value * (i32::MAX as f32)) as i32
}

#[inline(always)]
pub fn i32_to_f32(value: i32) -> f32 {
    (value as f32) / i32::MAX as f32
}

#[inline(always)] 
pub fn f64_to_i32(value: f64) -> i32 {
    (value * (i32::MAX as f64)) as i32
}

#[inline(always)]
pub fn i32_to_f64(value: i32) -> f64 {
    (value as f64) / i32::MAX as f64
}

#[allow(unused)]
fn encode_alaw(value: i16) -> u8 {
    todo!();
    let sign_bit = if value < 0 { 0x80 } else { 0 };
    let abs_sample = if value < 0 { -value } else { value };

    let compressed_value = if abs_sample > 32635 {
        0x7F
    } 
    else if abs_sample >= 256 {
        let exponent = (abs_sample.leading_zeros() - 23) as i16;
        let mantissa = (abs_sample >> (exponent + 3)) & 0x0F;
        ((exponent << 4) | mantissa) as u8
    } 
    else {
        (abs_sample >> 4) as u8
    };

    !((compressed_value | sign_bit) as u8)
}

#[allow(unused)]
fn decode_alaw(value: u8) -> i16 {
    todo!();
    let alaw_sample = !value;
    let sign = if alaw_sample & 0x80 != 0 { -1 } else { 1 };
    let exponent = (alaw_sample & 0x70) >> 4;
    let mantissa = (alaw_sample & 0x0F) as i16;

    let decoded_sample = if exponent == 0 {
        (mantissa << 4) + 8
    } else {
        ((mantissa << 4) | 0x100) << (exponent - 1)
    };

    sign * decoded_sample
}