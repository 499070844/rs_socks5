use server::lsocks5::{Socks5, Auth, Methods};

fn main() {
    let mut socks5 = Socks5::new();
    socks5.auth(Auth { user:String::from("aaa"), pw: String::from("bbb")});
    socks5.method(Methods::UserPass);
    socks5.start();
}
