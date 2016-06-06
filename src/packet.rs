

#[derive(Debug)]
pub struct Packet {
    header : Header,
    payload: Vec<u8>, // Bytes
}


impl Packet {
    pub fn new() -> Result<Packet> {

    }
    pub fn parse_header(buf: &[u8]) -> Result<usize> {
        // buf: [0; 12], 12 Bytes( 96 Bits )

    }
    pub fn parse(buf: &[u8]) -> Result<Packet> {

    }
    pub fn from_bytes() -> Result<Packet> {
        
    }
    pub fn to_bytes(&self) -> Result<[u8]> {

    }
}