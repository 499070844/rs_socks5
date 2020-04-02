use std::net::{TcpListener,TcpStream};

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream);
    }
}

fn handle_request(mut stream:TcpStream) {
    use std::io::prelude::*;
    let mut buffer = [0; 3];

    stream.read(&mut buffer).unwrap();

    let response = res1(&buffer);
    println!("{:?}", response);

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

fn res1(req: &[u8]) -> [u8;2] {
    let mut result = [0;2];

    

    result[0] = 0x05 as u8;

    if req[2] != Methods::NoAuth as u8 {
        result[1] = Methods::NoReturn as u8;
    } else {
        result[1] = Methods::NoAuth as u8;
    }
    result
}

enum Socks5 {
    Ver = 0x05,
}

enum Methods {
    NoAuth = 0x00,
    GssAPI = 0x01,
    UserPass = 0x02,
    IanaU = 0x03,
    IanaD = 0x7F,
    NoReturn = 0xFF
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
