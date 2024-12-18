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

fn save_with_houd(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(path)?;
    let samples: Vec<f32> = reader.samples().map(|s| s.unwrap()).collect();
    
    let mut writer = hound::WavWriter::create("test_hound.wav", reader.spec())?;
    
    samples.iter().for_each(|s| writer.write_sample(*s).unwrap());

    Ok(())
}

fn save_with_l3gion(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut decoder = wav::LgWavDecoder::new(path)?;
    let samples: Vec<f32> = decoder.samples().collect();
    
    let mut encoder = wav::LgWavEncoder::new("test_l3gion.wav", decoder.format())?;
    
    samples.iter().for_each(|s| encoder.encode_sample(*s).unwrap());
    
    Ok(())
}

fn main() -> Result<(), error::Error> {
    // for sample in SAMPLES {
        let sample = std::format!("samples/{}.wav", SAMPLES[4]);
        // save_with_houd(&sample).unwrap();
        save_with_l3gion(&sample).unwrap();
    // }
    
    Ok(())
}
