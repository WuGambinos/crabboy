use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct SerialOutput {
    buffer: Vec<u8>,
}

impl SerialOutput {
    pub fn new() -> SerialOutput {
        SerialOutput { buffer: Vec::new() }
    }

    pub fn write_byte(&mut self, data: u8) {
        self.buffer.push(data);
    }

    pub fn read_bytes(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn output(&mut self) {
        let result = String::from_utf8(self.buffer.clone());

        match result {
            Ok(s) => print!("{}", s),
            Err(e) => println!("Error: {}", e),
        }

        self.buffer.clear();
    }
}
