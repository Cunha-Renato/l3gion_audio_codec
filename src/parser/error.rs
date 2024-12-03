use crate::reader::error::LgReadAudioFileErr;

pub enum LgAudioParseErr {
    PARSE(String),
    FILE(LgReadAudioFileErr),
}
impl std::fmt::Display for LgAudioParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LgAudioParseErr::PARSE(err_msg) => write!(f, "Parse Error: {err_msg}"),
            LgAudioParseErr::FILE(f_e) => write!(f, "{f_e}"),
        }
    }
}
impl std::fmt::Debug for LgAudioParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl std::error::Error for LgAudioParseErr {}
impl From<String> for LgAudioParseErr {
    fn from(value: String) -> Self {
        Self::PARSE(value)
    }
}
impl From<LgReadAudioFileErr> for LgAudioParseErr {
    fn from(value: LgReadAudioFileErr) -> Self {
        Self::FILE(value)
    }
}