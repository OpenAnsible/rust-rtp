

// A set of default mappings for audio and video is specified in the companion RFC 3551.
// http://tools.ietf.org/html/rfc3551
// http://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-1

#[derive(Debug)]
pub enum PayloadType {
  vp8,
  ogg,
  mp3,
  mpeg4,
  h264
}