use server::lsocks5::Socks5;

fn main() {
    Socks5::<u8>::start("127.0.0.1:10808");
}

/* fn future() {
    let config = Config.peek("XXX");
    let socks5 = config.set_into_socks5();
    socks.start();
    ||
    Socks5.socket("127.0.0.1:xxxx");
    Socks5.method("NoAuth");
    Socks5.auth("ddlin","xxxxxx");
    Socks5.start();
    ||
    SOcks5.socket("config.socket");
    socks5.method("config.method");
    socks5.auto("config.user","config.pw");
} */