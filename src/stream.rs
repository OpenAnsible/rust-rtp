
use std::net::{ UdpSocket, TcpStream, SocketAddr };

#[derive(Debug)]
pub enum Stream {
    Tcp(TcpStream),
    Udp(UdpSocket)
}

impl Stream {
    pub fn read(&self, buf: &mut [u8]) -> Result<(usize, Option<SocketAddr>)> {
        match *self {
            Stream::Tcp(ref tcp_stream) => {

            },
            Stream::Udp(ref udp_socket) => match *udp_socket.recv_from(buf) {
                
            }
        }
    }
    pub fn write(&self, buf: &[u8]) -> Result<usize> {

    }
}