use crate::{file_parser::LgAudioFileParser, reader::{self, error::LgReadFileErr}, wav::chunk::fmt::WavFmtChunk};

const FMT: &str = "fmt ";
const FACT: &str = "fact";
const DATA: &str = "data";

#[derive(Debug, Default, Clone, Copy)]
pub struct LgWavParser {}
impl LgAudioFileParser for LgWavParser {
    type T = Result<(), LgReadFileErr>;

    fn parse(&mut self, path: impl AsRef<str>) -> Self::T {
        let bytes = reader::read_file(path, "wav, wave")?;

        if self.header_valid(&bytes[..12]) {
            let chunks = self.parse_chunks(bytes);
            for (ck_id, ck_data) in chunks {
                println!("{ck_id}, {}", ck_data.len());
                match ck_id.as_str() {
                    FMT => println!("{:#?}", WavFmtChunk::from(ck_data.len(), &ck_data)),
                    _ => (),
                }
            }
        }
        
        Ok(())
    }
}
impl LgWavParser {
    fn header_valid(&self, bytes: &[u8]) -> bool {
        match (
            &*String::from_utf8_lossy(&bytes[..4]), 
            &*String::from_utf8_lossy(&bytes[8..])
        ) {
            ("RIFF", "WAVE") => true,
            _ => true,
        }
    }
    
    // Returns all the valid chunks (id, data).
    fn parse_chunks(&self, bytes: Vec<u8>) -> Vec<(String, Vec<u8>)> {
        let mut result = Vec::default();
        let mut cursor = 12;

        while cursor < bytes.len() {
            // Parsing the chunk id and it's size
            let ck_id = String::from_utf8_lossy(&bytes[cursor..cursor + 4]);
            let ck_size = u32::from_le_bytes([
                bytes[cursor + 4], 
                bytes[cursor + 5], 
                bytes[cursor + 6], 
                bytes[cursor + 7]
            ]);
            cursor += 8;

            // The rest of the chunk is the chunk data.
            result.push((
                ck_id.to_string(), 
                bytes[cursor..cursor + ck_size as usize].to_vec()
            ));

            // More info could be stored in the file, but we don't care, so as soon as we
            // see the data chunk we end parsing
            if &ck_id.to_string() == DATA { return result; }

            cursor += ck_size as usize;
        }

        result
    }
}