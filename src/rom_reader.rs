use std::fs::File;
use std::io::Read;

pub struct Reader;

impl Reader {
    pub fn read_rom(path: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}