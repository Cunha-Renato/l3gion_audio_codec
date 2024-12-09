use lg_audiof::{wav::{decode_bytes, parser::LgWavRaw}, LgAudioFileParser};

fn main() {
    let file_names = [
        "samples/m1f1_alaw.wav", 
        "samples/m1f1_mulaw.wav", 
        "samples/m1f1_alaw_we.wav", 
        "samples/soldiers_eyes.wav", 
        "samples/soldiers_eyes_pcm_24.wav", 
        "samples/sine_pcm.wav"
    ];

    let mut dec = LgWavRaw::default()
        .parse(file_names[4])
        .unwrap()
        .decode_with(|bytes, fmt, _| {
            let sample = decode_bytes(bytes, fmt.fmt_tag, fmt.bits_per_sample).ok()?;
            Some(sample as i16)
        })
        .unwrap();
}
