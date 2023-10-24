use std::io;
use std::io::Read;
use std::fs::File;

fn main() {、
    //Rust 有 panic! 宏。当执行这个宏时，程序会打印
    //出一个错误信息，展开并清理栈数据，然后接着退出
    println!("Hello, world!");
    panic!("crash and burn");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
    //简写
    let f = File::open("hello.txt").unwrap();
    //传播（propagating）错误
    //为比起你代码所拥有的
    //上下文，调用者可能拥有更多信息或逻辑来决定应该如何处理错误。
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
        }
        
}
