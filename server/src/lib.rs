use std::net::{TcpListener, TcpStream};

pub mod lsocks5;



//处理链接关闭
//tcp面向字节流 -> 系统帮你完成(校验顺序 -> 系统切割数据 -> 重传 -> 流量控制)
//udp面向数据包 -> 需校验 -> 可广播
/* pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("listen:{}", stream.peer_addr().unwrap());
        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream) {
    use std::io::prelude::*;
    loop {
        let mut i = 1;
        let mut buffer = [0; 32];

        stream.read(&mut buffer).unwrap();
        println!("req:{:?}", buffer);

        let response = res(i,&buffer);
        println!("res:{:?}", response);

        //若是0xFF就不写入了，不发送
        if response[1] != 0xFF {
            stream.write(&response).unwrap();
        }
        stream.flush().unwrap();
        i += 1;
        continue;
    }
}

fn res(i:u8,req:&[u8]) -> &[u8] {
    match i {
        1 => res1(req),
        2 => res2(req),
    }
}
 */
// 建立链接，客户端发来的request, 包含多个字节， 第一字节是版本号，第二字节是方法的种类，第三字节是方法
//    +----+----------+----------+
//    |VER | NMETHODS | METHODS  |
//    +----+----------+----------+
//    | 1  |    1     |  1~255   |
//    +----+----------+----------+
/* fn res1(req: &[u8]) -> &[u8] {
    use std::collections::HashMap;
    let mut result = [0; 2];

   let [ver,n_methods] = *req;
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

    &result[..]
}

fn res2(req: &[u8]) -> &[u8] {
    let [ver,cmd,_,a_type] = *req;

    &[1] //麻烦
}
 */
/* struct DST {
    dst_addr: String,
    dst_port: [u8;2],
}
impl DST {
    fn new(a_type: AddrType, input: &[u8]) -> DST {
        let addr_vec = Vec::new();
        let addr = match a_type {
            AddrType::Ip4 => {
                let a = &input[4..9];
            },
            AddrType::Ip6 => Ipv6Addr::new()
            AddrType::Domain => {
                let n = input[4];
                String::from_utf8_lossy(&input[5..(6+n) as usize])
            }
        };
        DST {
            dst_addr: addr,
            dst_port: [2,4]
        }
    }    
}

enum AddrType {
    Ip4,
    Ip6,
    Domain,
}
impl AddrType {
    fn new(a_type: u8) -> AddrType {
        match a_type {
            0x01 => AddrType::Ip4,
            0x04 => AddrType::Ip6,
            0x03 => AddrType::Domain
        }
    }
}

 */
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

/* 
    Sock5.start(&self) {
        let listener = Tcplistener::bind(Socks5.socket).unwrap();
        for stream in listener.incoming().unwrap() {
            let stream = stream.unwrap();
            Socks5.handle(stream);
        }
    }
*/


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
