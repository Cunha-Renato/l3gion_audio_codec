pub mod error;

use std::{io::Read, ops::Deref};
use error::LgReadAudioFileErr;

#[derive(Debug, Default, Clone)]
pub struct LgVecReader<T> {
    cursor: usize,
    data: Vec<T>
}
impl<T> From<Vec<T>> for LgVecReader<T> {
    fn from(data: Vec<T>) -> Self {
        Self {
            cursor: 0,
            data
        }
    }
}
impl<T> From<LgVecReader<T>> for Vec<T> {
    fn from(value: LgVecReader<T>) -> Self {
        value.inner()
    }
}
impl<T> Deref for LgVecReader<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> LgVecReader<T> {
    pub fn inner(self) -> Vec<T> {
        self.data
    }
    
    /// If (cursor + n) >= capacity, it will read_to_end.
    pub fn read_quantity(&mut self, n: usize) -> &[T] {
        let result = if (self.cursor + n) >= self.data.len() {
            return self.read_to_end();
        }
        else {
            &self.data[self.cursor..self.cursor + n]
        };

        self.cursor += n;
        
        result
    }

    pub fn read_to_end(&mut self) -> &[T] {
        let result = &self.data[self.cursor..];
        self.cursor = self.data.len();
        
        result
    }

    /// Will panic if n >= capacity.
    pub fn skip_quantity(&mut self, n: usize) {
        self.cursor += n;
    }
    
    pub fn reach_end(&self) -> bool {
        self.cursor >= self.data.len()
    }
}

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