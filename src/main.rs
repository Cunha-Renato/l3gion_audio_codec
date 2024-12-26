use std::{fmt::Debug, io::BufWriter};

use hound::WavSpec;
use l3gion_audio_codec::{decoder::LgDecoder, encoder::LgEncoder, error, wav::{self, LgWavDecoder, WavFmt, WavFmtTag}, Sample};

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

fn save_with_houd(samples: Vec<impl hound::Sample + Copy>, format: WavSpec) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = hound::WavWriter::create("test_hound.wav", format)?;
    
    samples.iter().for_each(|s| writer.write_sample(*s).unwrap());

    Ok(())
}

fn save_with_l3gion(samples: Vec<impl Sample>, format: WavFmt) -> Result<(), Box<dyn std::error::Error>> {
    let mut encoder = wav::LgWavEncoder::new("test_l3gion.wav", format)?;
    
    samples.into_iter().for_each(|s| encoder.encode_sample(s).unwrap());
    encoder.flush()?;
    
    Ok(())
}

fn open_hound(path: &str) -> Result<hound::WavReader<impl std::io::Read>, Box<dyn std::error::Error>> {
    Ok(hound::WavReader::open(path)?)
}

fn open_l3gion(path: &str) -> Result<LgWavDecoder<impl std::io::Read>, Box<dyn std::error::Error>> {
    Ok(LgWavDecoder::new(path)?)
}

fn test_write() -> Result<(), Box<dyn std::error::Error>> {
    let samples: Vec<i32> = vec![100000, 40, 50, 60, 70];
    let mut test_file = BufWriter::new(std::fs::File::create("test_write.wav")?);
    
    samples.into_iter().for_each(|s| s.write(&mut test_file, l3gion_audio_codec::SampleType::INT, 32).unwrap());

    Ok(())
}

fn test_l3gion() -> Result<(), Box<dyn std::error::Error>> {
    // let path = std::format!("samples/{}.wav", SAMPLES[3]);
    let path = "test_l3gion.wav";

    let mut dec = open_l3gion(&path).unwrap();
    let mut reader = open_hound(&path).unwrap();
    let mut fmt = dec.info();
    fmt.format = WavFmtTag::WAVE_FORMAT_PCM;
    
    println!("L3gion FMT: {:#?}", dec);
    println!("Hound Specs: {:#?}", reader.spec());
    
    let l_samples: Vec<i32> = dec.samples().collect();
    let h_samples: Vec<i32> = reader.samples().map(|s| s.unwrap()).collect();

    println!("L3gion samples: {}", dec.len());
    println!("Hound samples: {}", reader.len());

    save_with_houd(l_samples, reader.spec())?;
    save_with_l3gion(h_samples, dec.info())?;

    Ok(())
}

fn main() {
    test_l3gion().unwrap();
}
