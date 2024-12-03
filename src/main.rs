use lg_audiof::{wav::parser::LgWavParser, LgAudioFileParser};

fn main() {
    let a = LgWavParser::default()
        .parse("samples/m1f1_alaw.wav")
        .unwrap();
    
    println!("{:#?}", a);
}
