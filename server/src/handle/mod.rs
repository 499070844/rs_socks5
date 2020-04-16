use std::net::TcpStream;
use std::process;

pub mod socks;
use socks::{First, Second, HandleSocks5, Items, Socks5Req};

/// Tcp only
pub struct Handle {
    stream: TcpStream,
}

impl Handle {
    pub fn new(stream: TcpStream) -> Self {
        Handle { stream }
    }

    ///读取stream内容，返回Result<Vec<u8>>
    fn read(&mut self) -> Result<Vec<u8>, ()> {
        use std::io::prelude::Read;
        let mut buffer: [u8; 1024] = [0; 1024];
        let read_len = self.stream.read(&mut buffer);
        match read_len {
            Ok(len) => {
                let mut result = Vec::new();
                for i in 0..=len + 1 {
                    result.push(buffer[i]);
                }
                Ok(result)
            }
            Err(_) => {
                println!("Error: fail to read stream");
                Err(())
            }
        }
    }

    pub fn send(&mut self, content: &[u8]) -> bool {
        use std::io::prelude::Write;
        let writen_len = self.stream.write(content);

        match writen_len {
            Err(_) => { println!("Error: fail to  write data");
                return false;
            }
            Ok(_) => {
                return true;
            }
        }
    }
}

impl HandleSocks5 for Handle {
    fn read_req(&mut self, status: u8) -> Result<Items,()> {
        let raw_req = self.read();

        if let Ok(raw_req) = raw_req {
            let req = match status {
                1 => First::from_vec(raw_req),
                2 => Second::from_vec(raw_req),
                _ => {
                    println!("这个地方只能填1或者2");
                    process::exit(1);
                }
            };
            if let Ok(req) = req {
                return Ok(req);
            }
        };
        Err(())
    }
}
