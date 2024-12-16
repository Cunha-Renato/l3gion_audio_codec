use l3gion_audio_codec::{decoder::LgDecoder, encoder::LgEncoder, error, wav};

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
    // for sample in SAMPLES {
        let sample = std::format!("samples/{}.wav", SAMPLES[2]);

        let mut wav = wav::LgWavDecoder::new(sample)?;
        println!("{:#?}", wav);
        let samples: Vec<i32> = wav.samples().collect();
        
        let mut wav_enc = wav::LgWavEncoder::new("test.wav", wav.format())?;
        
        samples.iter().for_each(|s| wav_enc.encode_sample(*s).unwrap());
    // }
    
    Ok(())
}
