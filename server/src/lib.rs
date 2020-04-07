use std::net::{TcpListener, TcpStream};

pub mod lsocks5;



//处理链接关闭
//tcp面向字节流 -> 系统帮你完成(校验顺序 -> 系统切割数据 -> 重传 -> 流量控制)
//udp面向数据包 -> 需校验 -> 可广播

// 建立链接，客户端发来的request, 包含多个字节， 第一字节是版本号，第二字节是方法的种类，第三字节是方法
//    +----+----------+----------+
//    |VER | NMETHODS | METHODS  |
//    +----+----------+----------+
//    | 1  |    1     |  1~255   |
//    +----+----------+----------+
/* fn res1(req: &[u8]) -> &[u8] {
    use std::collections::HashMap;
    let mut result = [0; 2];

   let [ver,n_methods] = *req;
    //具体每种方法
    let mut methods: HashMap<u8, _> = HashMap::with_capacity(n_methods as usize);

    for i in 2..2 + n_methods {
        let i = i as usize;
        methods.insert(req[i], 0);
    }

    //验证是否socks5请求
    if ver != Socks5::Ver as u8 && n_methods < 1 {
        result = [0x05, 0xFF];
    }

    // 返回第一个是固定的
    result[0] = 0x05 as u8;

    //确定socks5请求种类
    let res_method: u8 = match methods.contains_key(&(Methods::NoAuth as u8)) {
        true => Methods::NoAuth as u8,
        false => Methods::NoReturn as u8,
    };

    //写入验证类型
    result[1] = res_method;

    &result[..]
}
*/

enum Socks5 {
    Ver = 0x05,
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
