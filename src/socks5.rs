use std::net::TcpStream;


pub struct Socks5<'a> {
    ver: u8,
    stream: TcpStream,
    methods: Methods,
    addr: Option<Addr>,
}

impl Socks5<_> {
    pub fn new(stream: TcpStream) -> Self {
        Socks5 {
            ver: 0x05,
            stream,
            addr: None,
            methods: Methods::NoReturn 
        }
    }
}



struct Addr {
    addr: Eaddr
}

enum Eaddr {
    Ipv4,
    Ipv6,
    Domain
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