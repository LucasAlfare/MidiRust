#![allow(unused)]

mod my_reader;
mod header;
mod event;
mod meta_event;

use std::{
    fs::{self, File},
    io::Read,
};

use crate::my_reader::MyReader;
use crate::header::Header;

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
