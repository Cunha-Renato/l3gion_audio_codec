pub mod error;

use std::{io::Read, ops::{Deref, DerefMut}};
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
impl<T> DerefMut for LgVecReader<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<T> LgVecReader<T> {
    pub fn inner(self) -> Vec<T> {
        self.data
    }
    
    /// Errors if (cursor + n) > capacity || cursor >= capacity.
    pub fn read_quantity(&mut self, n: usize) -> std::io::Result<&[T]> {
        self.check_n(n-1)?;
        self.check_n(0)?;

        let result = &self.data[self.cursor..self.cursor + n];
        self.cursor += n;
        
        Ok(result)
    }

    /// Errors if cursor >= capacity.
    pub fn read_to_end(&mut self) -> std::io::Result<&[T]> {
        self.check_n(0)?;
        let result = &self.data[self.cursor..];
        self.cursor = self.data.len();
        
        Ok(result)
    }

    /// Errors if (cursor + n) >= capacity.
    pub fn skip_quantity(&mut self, n: usize) -> std::io::Result<()> {
        self.check_n(n)?;
        self.cursor += n;
        
        Ok(())
    }
    
    pub fn reach_end(&self) -> bool {
        self.check_n(0).is_err()
    }

    pub fn take_quantity(&mut self, n: usize) -> Self {
        let cursor = self.cursor;

        let mut result = self.split_off(cursor);
        let mut right = result.split_off(n);

        self.data.append(&mut right);

        result.into()
    }
}
impl<T> LgVecReader<T> {
    fn check_n(&self, n: usize) -> std::io::Result<()> {
        if (self.cursor + n) >= self.len() {
            return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
        }
        
        Ok(())
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