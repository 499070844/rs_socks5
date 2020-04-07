use std::net::{TcpListener, TcpStream};

pub struct Socks5 {
    ver: u8,
    auth: Option<Auth>,
    method: Methods,
    socket: &'static str,
}

impl Socks5 {
    /// 初始化Socks5
    /// auth:None,method:0x00,socket: localhost:20480
    pub fn new() -> Socks5 {
        Socks5 {
            ver: 0x05,
            auth: None,
            method: Methods::NoAuth,
            socket: "127.0.0.1:20480",
        }
    }

    pub fn auth(&mut self, auth: Auth) {
        self.auth = Some(auth);
    }

    pub fn method(&mut self, method: Methods) {
        self.method = method;
    }

    pub fn socket(&mut self, a: &'static str) {
        self.socket = a;
    }

    pub fn start(&self) {
        use std::net::{TcpListener, TcpStream};
        use std::process;
        use Methods;

        match self.auth {
            None => {}
            _ => match self.method {
                Methods::UserPass => {}
                _ => {
                    println!("When Method is UserPass, set auth");
                    process::exit(1);
                }
            },
        }


        let listener = TcpListener::bind(self.socket).unwrap();
        println!("Server is listening at {}",self.socket);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                  //  handle(stream)
                },
                Err(e) => println!("{:#}",e),
            }
        }
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
pub enum Methods {
    NoAuth = 0x00,
    GssAPI = 0x01,
    UserPass = 0x02,
    IanaU = 0x03,
    IanaD = 0x7F,
    NoReturn = 0xFF,
}
/// 当Method为UserPass(0x02)时 用户名和密码才生效
pub struct Auth {
    pub user: String,
    pub pw: String,
}
