#[allow(non_camel_case_types)]
pub enum LgReadAudioFileErr {
    NO_EXTENTION,
    /// Expected, Found.
    WRONG_EXTENSION(String, String),
    IO(std::io::Error)
}
impl From<std::io::Error> for LgReadAudioFileErr {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
impl std::fmt::Display for LgReadAudioFileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LgReadAudioFileErr::NO_EXTENTION => write!(f, "File does not have an extension!"),
            LgReadAudioFileErr::WRONG_EXTENSION(expected, found) => write!(f, "Wrong file extension, expected: {expected}, found: {found}!"),
            LgReadAudioFileErr::IO(err) => write!(f, "{err}"),
        }
    }
}
impl std::fmt::Debug for LgReadAudioFileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl std::error::Error for LgReadAudioFileErr {}