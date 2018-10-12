use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

pub fn read(reader: &mut TcpStream) -> Result<String, String> {
    let mut buf_str = String::new();
    match BufReader::new(reader).read_line(&mut buf_str) {
        Ok(_) => Ok(buf_str),
        Err(err) => Err(err.to_string()),
    }
}

pub fn write(writer: &mut TcpStream, word: &str) -> Result<(), String> {
    match BufWriter::new(writer).write(format!("{}\n", word).as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
