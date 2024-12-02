use lg_audiof::{wav::parser::LgWavParser, LgAudioFileParser};

fn main() {
    LgWavParser::default()
        .parse("samples/m1f1_mulaw.wav").unwrap();
}
