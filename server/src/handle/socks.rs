
//TODO:这个函数要返回东西，现在还mei xiang hao fan hui shen me dongxi
pub trait HandleSocks5 {
    fn read_req(&mut self, status: u8) ;
}
pub trait Socks5Req {
    fn from_vec(sth: Vec<u8>) -> Result<Self,()> where Self: Sized;
}

enum Items {
    A(First),
    B(),
}
use crate::lsocks5::{Cmd, Methods};
#[derive(Debug)]
pub struct First {
    ver: u8,
    n_method: Methods,
    methods: Vec<u8>,
}
impl Socks5Req for First {
    fn from_vec(sth: Vec<u8>) -> Result<First, ()> {
        let ver = sth.get(0);
        let n_method = sth.get(1);

        if let Some(ver) = ver {
            if let Some(n_method) = n_method {
                let methods = sth.get(2..(2 + n_method) as usize);
                if let Some(methods) = methods {
                    return Ok(First {
                        ver: *ver,
                        n_method: Methods::new(*n_method),
                        //TODO:to_vec()是一个Copy方法，以后想想不用Copy怎么实现
                        methods: methods.to_vec(),
                    });
                }
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