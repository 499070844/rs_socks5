Tcp -> 面向字节流: 系统底层解决 重传，包乱序，包切割等问题。三次握手后，stream将建立链接
Udp -> 面向数据块： 你想数据块多大，udp就发多大，系统底层不介入，接受到也是多大。但是需要解决重传，乱序等问题，适合做广播