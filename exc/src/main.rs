use server::lsocks5::{Socks5, Auth};
use server::Methods;

fn main() {
    let mut socks5 = Socks5::new();
    socks5.method(Methods::NoAuth);
    socks5.start();
}
