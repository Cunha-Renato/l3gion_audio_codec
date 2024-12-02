pub trait LgAudioFileParser {
    type T;

    fn parse(&mut self, path: impl AsRef<str>) -> Self::T;
}