
pub trait HandleSocks5 {
    fn read_req(&mut self, status: u8) -> Result<Items,()>;
}
pub trait Socks5Req {
    fn from_vec(sth: Vec<u8>) -> Result<Self,()> where Self: Sized;
}

//TODO:B STATus 2 即是 B还没写
#[derive(Debug)]
pub enum Items {
    A(First),
    B(),
}

//    +----+----------+----------+
//    |VER | NMETHODS | METHODS  |
//    +----+----------+----------+
//    | 1  |    1     |  1~255   |
//    +----+----------+----------+
#[derive(Debug)]
pub struct First {
    pub ver: u8,
    pub n_method: Methods,
    pub methods: Vec<u8>,
}
//NOTE: 验证request东一个西一个的，不好管理，还是要一个地方统一写验证规则
impl Socks5Req for First {
    fn from_vec(sth: Vec<u8>) -> Result<First, ()> {
        let ver = sth.get(0);
        let n_method = sth.get(1);

        if let Some(ver) = ver {
            //ver： 版本为5
            if ver == &0x05 {

            if let Some(n_method) = n_method {
                let methods = sth.get(2..(2 + n_method) as usize);
                if let Some(methods) = methods {
                    return Ok(First {
                        ver: *ver,
                        n_method: Methods::new(*n_method),
                        //TODO:to_vec()是一个Copy方法，以后想想不用Copy怎么实现
                        methods: methods.to_vec(),
                    });
                }}
            }
        }
        return Err(());
    }
}
struct Second {
    ver: u8,
    cmd: u8,
    rsv: u8,
    a_type: u8,
    dst_addr: u8,
    dst_port: [u8; 2],
}


/// 方法的种类：
///    + NoAuth:0x00: NO AUTHENTICATION REQUIRED
///    + GssApi:0x01: GSSAPI
///    + UserPass:0x02: USERNAME/PASSWORD
///    + IanaU,IanaD:0x03: to X’7F’ IANA ASSIGNED
///    + 0x80: to X’FE’ RESERVED FOR PRIVATE METHODS
///    + NoReturn:0xFF: NO ACCEPTABLE METHODS
/// 其中IanaU,D是区间 从 0x03~0x7F
#[derive(Debug)]
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



pub enum Cmd {
    Connect = 0x01,
    Bind = 0x02,
    Udp = 0x03,
}
impl Cmd {
    //FIXME: 这个地方可能要做错误处理
    pub fn new(n: u8) -> Self {
        match n {
            0x01 => Self::Connect,
            0x02 => Self::Bind,
            0x03 => Self::Udp,
            _ => Self::Connect,
        }
    }
}