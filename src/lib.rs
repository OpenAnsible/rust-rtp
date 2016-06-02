
// A set of default mappings for audio and video is specified in the companion RFC 3551.
// http://tools.ietf.org/html/rfc3551
enum PayloadType {
  vp8,
  ogg,
  mp3,
  mpeg4,
  h264
}

#[derive(Debug)]
struct Header {
  version: usize,   // 2 bits
  padding: usize,   // 1 bit
  extension: usize, // 1 bit
  // csrc_count must be 0 to 15, If there are more than 15 contributing sources, only 15 can be identified
  // CSRC COUNT (Also is CC)
  csrc_count: usize, // 4 bits, The CSRC count contains the number of CSRC identifiers that follow the fixed header.
  marker   : usize, // 1 bit
  payload_type: PayloadType, // 7 bits
  sequence_number: usize,    // 16 bits
  timestamp : f64,           // 32 bits
  ssrc      : String,        // 32 bits, SHOULD be random
  csrc      : Vec<String>,   // 32 bits, 0 to 15 items ( The number of identifiers is given by the CC field),
}

// https://tools.ietf.org/html/rfc3550#section-5.3.1
#[derive(Debug)]
struct HeaderExtension {
    field: Type
}
