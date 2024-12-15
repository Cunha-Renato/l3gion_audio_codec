use l3gion_audio_codec::{error, wav};

const SAMPLES: [&str; 5] = [
    // "m1f1_alaw",
    // "m1f1_alaw_we",
    // "m1f1_mulaw",
    // "m1f1_mulaw_we",
    "m1f1_uint8",
    // "m1f1_int12",
    "m1f1_int16",
    "m1f1_int24",
    "m1f1_int32",
    "m1f1_float32"
];

fn main() -> Result<(), error::Error> {
    for sample in SAMPLES {
        let sample = std::format!("samples/{}.wav", sample);
        let mut h = hound::WavReader::open(&sample).unwrap();

        let mut wav = wav::LgWavDecoder::new(sample)?;
        for a in h.samples::<i32>() {
            let b = a.unwrap()+1;
        }
        println!("{:#?}", wav);
    }
    
    Ok(())
}
