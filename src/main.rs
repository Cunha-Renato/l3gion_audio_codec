use lg_audiof::{wav::parser::LgWavParser, LgAudioFileParser};

fn main() {
    let file_names = ["samples/m1f1_alaw.wav", "samples/m1f1_mulaw.wav","samples/m1f1_alaw_we.wav", "samples/soldiers_eyes.wav"];

    let a = LgWavParser::default()
        .parse(file_names[2])
        .unwrap();
    
    println!("{:#?}", a);
}
