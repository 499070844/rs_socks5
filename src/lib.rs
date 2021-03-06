use std::net::{TcpListener, TcpStream};

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("listen:{}",stream.peer_addr().unwrap());
        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream) {
    use std::io::prelude::*;
    let mut buffer = [0; 32];

    stream.read(&mut buffer).unwrap();
    println!("req:{:?}", buffer);

    let response = res1(&buffer);
    println!("res:{:?}", response);

    //若是0xFF就不写入了，不发送
    if response[1] != 0xFF {
        stream.write(&response).unwrap();
    }
    stream.flush().unwrap();

    let mut buffer = [0; 32];
    stream.read(&mut buffer).unwrap();
    println!("{:?}",buffer);

//    let response = res2(&buffer);

}

// 第一次握手，客户端发来的request, 包含多个字节， 第一字节是版本号，第二字节是方法的种类，第三字节是方法
//    +----+----------+----------+
//    |VER | NMETHODS | METHODS  |
//    +----+----------+----------+
//    | 1  |    1     |  1~255   |
//    +----+----------+----------+
fn res1(req: &[u8]) -> [u8; 2] {
    use std::collections::HashMap;
    let mut result = [0; 2];

    //socks版本
    let ver = req[0];
    //客户端提供多少种验证方法
    let n_methods = req[1];
    //具体每种方法
    let mut methods: HashMap<u8, _> = HashMap::with_capacity(n_methods as usize);

    for i in 2..2 + n_methods {
        let i = i as usize;
        methods.insert(req[i], 0);
    }

    //验证是否socks5请求
    if ver != Socks5::Ver as u8 && n_methods < 1 {
        result = [0x05, 0xFF];
    }

    // 返回第一个是固定的
    result[0] = 0x05 as u8;

    //确定socks5请求种类    
    let res_method: u8 = match methods.contains_key(&(Methods::NoAuth as u8)) {
        true => Methods::NoAuth as u8,
        false => Methods::NoReturn as u8,
    };

    //写入验证类型
    result[1] = res_method;

    result
}


/* fn res2(req: &[u8]) -> [u8; ] {

} */



enum Socks5 {
    Ver = 0x05,
}

/// 方法的种类：
///    + NoAuth:0x00: NO AUTHENTICATION REQUIRED
///    + GssApi:0x01: GSSAPI
///    + UserPass:0x02: USERNAME/PASSWORD
///    + IanaU,IanaD:0x03: to X’7F’ IANA ASSIGNED
///    + 0x80: to X’FE’ RESERVED FOR PRIVATE METHODS
///    + NoReturn:0xFF: NO ACCEPTABLE METHODS
/// 其中IanaU,D是区间 从 0x03~0x7F
enum Methods {
    NoAuth = 0x00,
    GssAPI = 0x01,
    UserPass = 0x02,
    IanaU = 0x03,
    IanaD = 0x7F,
    NoReturn = 0xFF,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
