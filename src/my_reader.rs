#![allow(unused)]

#[derive(Debug)]
pub struct MyReader {
    pub buffer: Vec<u8>,
    pub offset: u32,
}

impl MyReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        MyReader { buffer, offset: 0 }
    }

    pub fn read_1_byte(&mut self) -> u8 {
        let result = self.buffer[self.offset as usize];
        self.offset += 1;
        return result;
    }

    pub fn read_2_bytes(&mut self) -> u16 {
        let a = self.buffer[(self.offset + 0) as usize] as u16;
        let b = self.buffer[(self.offset + 1) as usize] as u16;

        let result = (a << 8) | b;

        self.offset += 2;

        return result;
    }

    pub fn read_4_bytes(&mut self) -> u32 {
        let a = self.buffer[(self.offset + 0) as usize] as u32;
        let b = self.buffer[(self.offset + 1) as usize] as u32;
        let c = self.buffer[(self.offset + 2) as usize] as u32;
        let d = self.buffer[(self.offset + 3) as usize] as u32;

        let result = (((((a << 24) | b) << 16) | c) << 8) | d;

        self.offset += 4;

        return result;
    }

    pub fn read_string(&mut self, length: u32) -> String {
        let mut result = String::new();

        for i in 0..length {
            let curr_byte = self.buffer[(self.offset + i) as usize];
            result.push(curr_byte as char);
        }

        self.offset += length;

        return result;
    }

    fn read_variable_length_value(&mut self) -> u32 {
        let mask: u8 = 0b0111_1111;
        let mut result_number: u32 = 0;
        let mut current_byte: u8;

        loop {
            current_byte = self.read_1_byte();
            result_number = (result_number << 7) | ((current_byte & mask) as u32);
            if current_byte >> 7 == 0 {
                return result_number;
            }
        }
    }
}

