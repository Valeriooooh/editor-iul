use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
pub enum FileError {
    WriteError(String),
    ReadError(String),
}

pub fn file_read(path: String) -> Result<String, FileError> {
    let mut str = String::new();
    let file = match File::open(path) {
        Ok(a) => a,
        Err(e) => return Err(FileError::ReadError(e.to_string())),
    };
    let mut buf = BufReader::new(file);
    match buf.read_to_string(&mut str) {
        Ok(_) => {}
        Err(e) => return Err(FileError::ReadError(e.to_string())),
    };
    Ok(str)
}

pub fn file_save(path: String, content: String) -> Result<(), FileError> {
    match std::fs::write(path, content) {
        Ok(_) => {}
        Err(e) => return Err(FileError::WriteError(e.to_string())),
    };
    Ok(())
}
