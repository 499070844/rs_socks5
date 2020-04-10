use super::handle::socks::{HandleSocks5,Methods,Items,First};
use super::handle::Handle;

pub struct Socks5 {
    ver: u8,
    auth: Option<Auth>,
    method: Methods,
    socket: &'static str,
}

impl Socks5 {
    /// 初始化Socks5
    /// auth:None,method:0x00,socket: localhost:20480
    pub fn new() -> Self {
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
        use std::net::TcpListener;
        use std::process;
        // 检查参数之间是否冲突
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

        //开始监听
        let listener = TcpListener::bind(self.socket).unwrap();
        println!("Server is listening at {}", self.socket);
        //处理字节流
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut handlee = Handle::new(stream);
                    let a = handlee.read_req(1).unwrap();
                    //TODO: socks5.handle_request(a);
                }
                Err(e) => println!("{:#}", e),
            }
        }
    }

    ///集中业务逻辑，把需要返回的，处理的逻辑放在这里，之后在start()里调用。&[u8] -> handle.send(&[u8])
    fn response1(&self, request: &First) -> &[u8] {
        if self.ver != request.ver {
            return &[0];
        }
        //TODO:let a = self.method.clone() as u8;

        for i in 0..request.methods.len() {
            
        }
        &[0]
    }
}

/// 当Method为UserPass(0x02)时 用户名和密码才生效
pub struct Auth {
    pub user: String,
    pub pw: String,
}


