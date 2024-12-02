#[allow(non_camel_case_types)]
pub enum LgReadFileErr {
    NO_EXTENTION,
    /// Expected, Found.
    WRONG_EXTENSION(String, String),
    IO(std::io::Error)
}
impl From<std::io::Error> for LgReadFileErr {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
impl std::fmt::Display for LgReadFileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LgReadFileErr::NO_EXTENTION => write!(f, "File does not have an extension!"),
            LgReadFileErr::WRONG_EXTENSION(expected, found) => write!(f, "Wrong file extension, expected: {expected}, found: {found}!"),
            LgReadFileErr::IO(err) => write!(f, "{err}"),
        }
    }
}
impl std::fmt::Debug for LgReadFileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl std::error::Error for LgReadFileErr {}