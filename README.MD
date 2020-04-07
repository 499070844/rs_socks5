Tcp -> 面向字节流: 系统底层解决 重传，包乱序，包切割等问题。三次握手后，stream将建立链接
Udp -> 面向数据块： 你想数据块多大，udp就发多大，系统底层不介入，接受到也是多大。但是需要解决重传，乱序等问题，适合做广播

伪代码
API 

```rs
fn future() {
    let config = Config.peek("XXX");
    let socks5 = config.set_into_socks5();
    socks.start();
  //  ||
    Socks5.socket("127.0.0.1:xxxx");
    Socks5.method("NoAuth");
    Socks5.auth("ddlin","xxxxxx");
    Socks5.start();
  //  ||
    SOcks5.socket("config.socket");
    socks5.method("config.method");
    socks5.auth("config.user","config.pw");
  //  ||
    Sockst.new(socket,method,auth);
    Sockst.start();
}

```

```rs
Sock5.start(&self) {
        let listener = Tcplistener::bind(Socks5.socket).unwrap();
        for stream in listener.incoming().unwrap() {
            let stream = stream.unwrap();
            Socks5.handle(stream);
        }
    }
```


那这个`handle`需要用到什么方法呢

确认是否socks5链接 `isSocks5()`,

确认认证类型 `isNeedAuth()`,

读取`GetRequest()`

socks5建立链接的迭代器?



    
