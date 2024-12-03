pub mod error;

pub trait LgAudioFileParser {
    type R;

    fn parse(&mut self, path: impl AsRef<str>) -> Self::R;
}