use std::net::{TcpStream, TcpListener};

pub struct Socks5 {
    ver: u8,
    rep: Option<Vec<u8>>,
    auth: Option<String>,
    method: u8,
}


impl Socks5 {
    pub fn start(host_port: &str)  {
        use std::net::{TcpListener, TcpStream};
        let listener = TcpListener::bind(host_port).unwrap();
        println!("Listening at {}", host_port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("From: {}", stream.peer_addr().unwrap());
        }
     }

    /* fn handle_request(&mut self, i: u8) {
        use std::io::prelude::*;
        let mut buffer = [0;32];
        self.stream.read(&mut buffer).unwrap();
        /* let rep = self.rep1();
        self.send(rep);
        self.handle_request(i+1)*/    
        println!("{:?}", buffer);
    } */
}



//************************************************************************
/* trait Send {
    fn send(&mut self, content:&[u8]) -> (); 
}
impl<T> Send for Socks5<T> {
    fn send(&mut self, content: &[u8]) {
        use std::io::prelude::*;
        self.stream.write(content).unwrap();
        self.stream.flush();
    }
} */