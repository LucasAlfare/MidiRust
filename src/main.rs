#![allow(unused)]

mod event;
mod header;
mod my_reader;
mod track;

use std::{
    fs::{self, File},
    io::Read,
};

use crate::header::Header;
use crate::my_reader::MyReader;

fn main() {
    let bytes = read_file_bytes("demo.mid");
    let mut reader = MyReader::new(bytes);

    let header = Header::new(&mut reader);

    println!("{:?}", header);
}

fn read_file_bytes(pathname: &str) -> Vec<u8> {
    let file_len = fs::metadata(pathname).unwrap().len();
    let mut buffer = vec![0; file_len as usize];
    let mut file = File::open(pathname).unwrap();
    let _ = file.read(&mut buffer);

    return buffer;
}
