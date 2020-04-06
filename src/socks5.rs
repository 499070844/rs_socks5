use std::net::{TcpStream, TcpListener};

pub struct Socks5 {
    ver: u8,
    stream: TcpStream,
}

impl Socks5 {
    pub fn start(host_port: &str) -> Self {
        let listener = TcpListener::bind(host_port).unwrap();
        let stream = listener.incoming().next().unwrap().unwrap();

        Socks5 {
            ver: 0x05,
            stream 
        }
    }
}



//************************************************************************
trait Send {
    fn send(&mut self, content:&[u8]) -> (); 
}
impl Send for Socks5 {
    fn send(&mut self, content: &[u8]) {
        use std::io::prelude::*;
        self.stream.write(content).unwrap();
        self.stream.flush();
    }
}