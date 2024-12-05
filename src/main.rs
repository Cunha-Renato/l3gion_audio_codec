use std::io::Write;

use lg_audiof::{wav::parser::LgWavParser, LgAudioFileParser};

fn main() {
    let file_names = [
        "samples/m1f1_alaw.wav", 
        "samples/m1f1_mulaw.wav", 
        "samples/m1f1_alaw_we.wav", 
        "samples/soldiers_eyes.wav", 
        "samples/soldiers_eyes_pcm_24.wav", 
        "samples/sine_pcm.wav"
    ];

    let _dec = LgWavParser::default()
        .parse(file_names[3])
        .unwrap()
        .decode()
        .unwrap();
    
    std::fs::File::create("test.wav").unwrap().write(&_dec.to_bytes().unwrap()).unwrap();
}
