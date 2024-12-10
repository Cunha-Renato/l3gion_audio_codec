use lg_audiof::{tools::*, wav::LgWav};

fn main() {
    let mut wav = LgWav::decode("samples/sine_pcm.wav").unwrap();
    println!("{:#?}", wav.fmt);

    let s: Vec<i32> = wav.samples().collect();
    println!("{}", wav.duration());
        
    let a = hound::WavReader::open("samples/sine_pcm.wav").unwrap();
    println!("{}", a.duration());
}
