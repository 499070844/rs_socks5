pub mod stream;

pub trait HandleSocks5 {
    fn read_req(&mut self, status: u8) -> ();
}

enum Items {
    A(First),
    B(),
}
use super::lsocks5::{Cmd, Methods};
#[derive(Debug)]
struct First {
    ver: u8,
    n_method: u8,
    methods: Vec<u8>,
}
impl First {
    pub fn from_vec(sth: Vec<u8>) -> Result<Self, ()> {
        let ver = sth.get(0);
        let n_method = sth.get(1);

        if let Some(ver) = ver {
            if let Some(n_method) = n_method {
                let methods = sth.get(2..(2 + n_method) as usize);
                if let Some(methods) = methods {
                    return Ok(First {
                        ver: *ver,
                        n_method: *n_method,
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
