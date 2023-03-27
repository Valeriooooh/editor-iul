use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn file_read(path: String) -> String {
    let mut str = String::new();
    let file = File::open(path).expect("erro a abrir ficheiro");
    let mut buf = BufReader::new(file);
    buf.read_to_string(&mut str).expect("erro a ler");
    str
}
