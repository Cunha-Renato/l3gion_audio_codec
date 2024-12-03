pub mod error;

use std::io::Read;
use error::LgReadAudioFileErr;

pub(crate) fn read_file(path: impl AsRef<str>, extension: impl AsRef<str>) -> Result<Vec<u8>, LgReadAudioFileErr> {
    let file_extension = std::path::Path::new(path.as_ref()).extension().ok_or(LgReadAudioFileErr::NO_EXTENTION)?;

    if !extension.as_ref()
        .contains(file_extension.to_string_lossy().to_string().as_str()) 
    {
        return Err(LgReadAudioFileErr::WRONG_EXTENSION(extension.as_ref().to_string(), file_extension.to_string_lossy().to_string()))
    }

    let mut file = std::fs::File::open(path.as_ref())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    Ok(buffer)
}