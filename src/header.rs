extern crate rand;
use rand::Rng;

/*
  The RTP header has the following format:

    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |V=2|P|X|  CC   |M|     PT      |       sequence number         |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                           timestamp                           |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |           synchronization source (SSRC) identifier            |
   +=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
   |            contributing source (CSRC) identifiers             |
   |                             ....                              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    

    Header Extension ( If the X bit in the RTP header is one )

    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |      defined by profile       |           length              |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                        header extension                       |
   |                             ....                              |

    Header extension (optional):
         The first 32-bit word contains a profile-specific identifier (16 bits) 
         and a length specifier (16 bits) that indicates 
         the length of the extension (EHL = extension header length) in 32-bit units, 
         excluding the 32 bits of the extension header.


RTP Data Header Validity Checks
    https://tools.ietf.org/html/rfc3550#appendix-A.1

*   RTP version field must equal 2.
*   The payload type must be known, and in particular it must not be equal to SR or RR.
*   If the P bit is set, then the last octet of the packet must contain a valid octet count, 
    in particular, less than the total packet length minus the header size.
*   The X bit must be zero if the profile does not specify that the
    header extension mechanism may be used.  Otherwise, the extension
    length field must be less than the total packet size minus the
    fixed header length and padding.
*   The length of the packet must be consistent with CC and payload
    type (if payloads have a known length).



*/



#[derive(Debug)]
pub struct Header {
  version: u8,      // 2 bits, RTP version field must equal 2. 
  padding: u8,      // 1 bit
  marker : u8,      // 1 bit
  pt     : u8,      // 7 bits, TODO: use enum.
  seq    : u16,     // 16 bits
  ts     : u32,     // 32 bits
  ssrc   : u32,     // 32 bits, SHOULD be random
  // CC( CSRC Count ): u8,   // 4 bits, The CSRC count contains the number of CSRC identifiers that follow the fixed header.
  csrc      : Vec<u32>,      // 32 bits, 0 to 15 items ( The number of identifiers is given by the CC field),
  extension : Option<Extension> // Extension Header
}

// https://tools.ietf.org/html/rfc3550#section-5.3.1
#[derive(Debug)]
pub struct Extension {
    identifier: u16,       // 16 bits
    length    : u16,       // 16 bits
    extension : Vec<u8>
}

pub struct SSRC(u32);
impl SSRC {
    pub fn new () -> SSRC {
        let mut rng = rand::thread_rng();
        SSRC(rng.gen::<u32>())
    }
    pub fn from_u32(n: u32) -> SSRC {
        SSRC(n)
    }
    pub fn to_u32(&self) -> u32 {
        self.0
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        let ssrc = format!("{:032b}", self.0);
        bytes.push(u8::from_str_radix(&ssrc[0..8], 2));
        bytes.push(u8::from_str_radix(&ssrc[8..16], 2));
        bytes.push(u8::from_str_radix(&ssrc[16..24], 2));
        bytes.push(u8::from_str_radix(&ssrc[24..32], 2));
        bytes
    }
}

pub struct CSRC(u32);
impl CSRC {
    pub fn new () -> CSRC {
        let mut rng = rand::thread_rng();
        CSRC(rng.gen::<u32>())
    }
    pub fn from_u32(n: u32) -> CSRC {
        CSRC(n)
    }
    pub fn to_u32(&self) -> u32 {
        self.0
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        let csrc = format!("{:032b}", self.0);
        bytes.push(u8::from_str_radix(&csrc[0..8], 2));
        bytes.push(u8::from_str_radix(&csrc[8..16], 2));
        bytes.push(u8::from_str_radix(&csrc[16..24], 2));
        bytes.push(u8::from_str_radix(&csrc[24..32], 2));
        bytes
    }
}


impl Header {
    pub fn new () -> Header {
        Header {
            version: 0x02,
            padding: 0x00,
            marker : 0x00,
            pt     : 0x00,
            seq    : 0x00,
            ts     : 0x00,
            ssrc   : SSRC::new(),
            csrc   : vec![],
            extension: None,
        }
    }
    pub fn parse () -> Result<Header> {
        
    }
    pub fn to_bytes(&self) -> Result<[u8]> {

    }
}



