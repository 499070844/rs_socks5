use crate::lsocks5::Socks5;


pub trait HandleSocks5 {
    fn read_req(&mut self, status: u8) -> Result<Items, ()>;
}


//TODO:B STATus 2 即是 B还没写
#[derive(Debug)]
pub enum Items {
    A(First),
    B(Second),
}

impl Items {
    pub fn response(self, socks5: &Socks5) -> Vec<u8> {
        match self {
            Items::A(first) => first.response(socks5),
            //Items::B(second) => second.response(socks5),
            _ => vec!(1,2,3),
        }
    }
}

//    +----+----------+----------+
//    |VER | NMETHODS | METHODS  |
//    +----+----------+----------+
//    | 1  |    1     |  1~255   |
//    +----+----------+----------+
#[derive(Debug)]
pub struct First {
    pub ver: u8,
    pub n_method: u8,
    pub methods: Vec<u8>,
}

pub trait Socks5Req where Self: Sized{
    fn from_vec(sth: Vec<u8>) -> Result<Items, ()>;
}

//NOTE: 验证request东一个西一个的，不好管理，还是要一个地方统一写验证规则
impl Socks5Req for First {
    fn from_vec(sth: Vec<u8>) -> Result<Items, ()> {
        let ver = sth.get(0);
        let n_method = sth.get(1);

        if let Some(ver) = ver {
            //ver： 版本为5
            if ver == &0x05 {
                if let Some(n_method) = n_method {
                    let methods = sth.get(2..(2 + n_method) as usize);
                    if let Some(methods) = methods {
                        return Ok(Items::A(First {
                            ver: *ver,
                            n_method: *n_method,
                            //FIXME:to_vec()是一个Copy方法，以后想想不用Copy怎么实现
                            methods: methods.to_vec(),
                        }));
                    }
                }
            }
        }
        return Err(());
    }
}

pub trait Socks5Res {
    fn response(&self, socks5: &Socks5) -> Vec<u8>; 
}

impl Socks5Res for First {
    fn response(&self, socks5: &Socks5) -> Vec<u8> {
        let s_method = *socks5.get_method() as u8;
        let method = &self.methods;
        let mut flag = false;
        for i in 0..method.len() {
            if method[i] == s_method {
                flag = true; 
            }
        }

        if flag {
            Vec::from(&[0x05,s_method][..])
        } else {
            Vec::from(&[0x05,0xFF][..])
        }
    }
}


//    +-----+-----+------+------+----------+----------+
//    | VER	| CMD | RSV  | ATYP	| DST.ADD  | DST.PORT |
//    +-----+-----+------+------+----------+----------+
//    | 1   |  1  | 0x00 |	 1  |  动态     |  2       |
//    +-----+-----+------+------+----------+----------+ 
#[derive(Debug)]
pub struct Second {
    ver: u8,
    cmd: Cmd,
    rsv: u8,
    a_type: u8,
    dst_addr: Addr
}
impl Socks5Req for Second {
    fn from_vec(sth: Vec<u8>) -> Result<Items, ()> {
        println!("second:{:?}", sth);
        let ver = sth.get(0);
        let cmd = sth.get(1);
        let a_type = sth.get(3);
        let addr = sth.get(4..);
        if let Some(ver) = ver {
            if let Some(cmd) = cmd {
                if let Some(a_type) = a_type {
                    if let Some(addr) = addr {
                        let cmd = Cmd::new(cmd);
                        //TODO: After finish Addr::new(&u8,)
                        let addr = Addr::new(a_type,addr);
                    }
                }
            }
        } 
        Err(())
    }
}

/// 方法的种类：
///    + NoAuth:0x00: NO AUTHENTICATION REQUIRED
///    + GssApi:0x01: GSSAPI
///    + UserPass:0x02: USERNAME/PASSWORD
///    + IanaU,IanaD:0x03: to X’7F’ IANA ASSIGNED
///    + 0x80: to X’FE’ RESERVED FOR PRIVATE METHODS
///    + NoReturn:0xFF: NO ACCEPTABLE METHODS
/// 其中IanaU,D是区间 从 0x03~0x7F
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Methods {
    NoAuth = 0x00,
    GssAPI = 0x01,
    UserPass = 0x02,
    IanaU = 0x03,
    IanaD = 0x7F,
    NoReturn = 0xFF,
}
impl Methods {
    pub fn new(n: u8) -> Methods {
        match n {
            0x00 => Methods::NoAuth,
            0x01 => Methods::GssAPI,
            0x02 => Methods::UserPass,
            0x03 => Methods::IanaU,
            0x7F => Methods::IanaD,
            0xFF => Methods::NoReturn,
            _ => Methods::NoReturn,
        }
    }
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Cmd {
    Connect = 0x01,
    Bind = 0x02,
    Udp = 0x03,
}
impl Cmd {
    //FIXME: 这个地方可能要做错误处理
    pub fn new(n: &u8) -> Self {
        match n {
            0x01 => Self::Connect,
            0x02 => Self::Bind,
            0x03 => Self::Udp,
            _ => Self::Connect,
        }
    }
}

///* 0x01：IPv4        
///* 0x03：域名
///* 0x04：IPv6
use std::net::{Ipv4Addr,SocketAddrV4};
use std::net::Ipv6Addr;
use std::net::SocketAddr;
use std::net::IpAddr;
#[derive(Debug)]
enum Addr {
    Ipv4(SocketAddr),
    Domain(String,u16),
    Ipv6(SocketAddr),
}
impl Addr {
    pub fn new(a_type: &u8,slice: &[u8]) -> Addr {
        match *a_type {
            0x01 => {
                let ipv4 = IpAddr::V4(Ipv4Addr::new(slice[0],slice[1],slice[2],slice[3]));
                let port = u16::from_be_bytes([slice[4],slice[5]]);
                let addr = SocketAddr::new(ipv4,port);
                Addr::Ipv4(addr)
            },
            0x04 => {
                let a = u16::from_be_bytes([slice[0],slice[1]]);
                let b = u16::from_be_bytes([slice[2],slice[3]]);
                let c = u16::from_be_bytes([slice[4],slice[5]]);
                let d = u16::from_be_bytes([slice[6],slice[7]]);
                let e = u16::from_be_bytes([slice[8],slice[9]]);
                let f = u16::from_be_bytes([slice[10],slice[11]]);
                let g = u16::from_be_bytes([slice[12],slice[13]]);
                let h = u16::from_be_bytes([slice[14],slice[15]]);

                let port = u16::from_be_bytes([slice[16],slice[17]]);
                let ipv6 = IpAddr::V6(Ipv6Addr::new(a,b,c,d,e,f,g,h));

                let addr = SocketAddr::new(ipv6,port);
                Addr::Ipv6(addr)
            },
            0x03 => {
                let len = slice.len();
                let domain_len = slice[0];
                let domain: Vec<u8> = Vec::from(&slice[1..=domain_len as usize]);
                //FIXME: 这里要做错误判断
                let domain = String::from_utf8(domain).unwrap();
                let port = u16::from_be_bytes([slice[len-2],slice[len-1]]);
                println!("domain:{}:{}",domain,port);
                Addr::Domain(domain,port)
            },
            //FIXME: 这里预计要返回一个Result<Addr,Err::nerver_mind>
            _ => Addr::Ipv4(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0), 1234)))
            
        }
    }
}
