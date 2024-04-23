use crate::my_reader::MyReader;

#[derive(Debug)]
pub struct Header {
    pub signature: String,
    pub length: u32,
    pub format: u16,
    pub num_tracks: u16,
    pub time_division: u16,
}

impl Header {
    pub fn new(reader: &mut MyReader) -> Self {
        Header {
            signature: reader.read_string(4),
            length: reader.read_4_bytes(),
            format: reader.read_2_bytes(),
            num_tracks: reader.read_2_bytes() - 1,
            time_division: reader.read_2_bytes()
        }
    }
}
