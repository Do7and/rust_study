use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("Hi number {} from spawned thread" , i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("Hey number {} from main thread",i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    


    let (tx,rx) = mpsc::channel();
    //tx.send(()).unwrap();

    let tx1 = mpsc::Sender::clone(&tx);
        //可以通过克隆发送者来创建多头
        
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    //send 函数获取其参数的所有权并移动这个值归接收者所有
    //通道的接收端有两个有用的方法： recv 和 try_recv
    //recv方法会阻塞主线程执行直到从通道中接收一个值。一旦发送了一个值， recv 会在一个 Result<T, E> 中返回它。当通道发送端关闭， recv 会返回一个错误表明不会再有新的值到来了。
    //try_recv 不会阻塞，相反它立刻返回一个 Result<T, E> ： Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
    
    



    //互斥器（mutex）是 “mutual exclusion” 的缩写
    use std::sync::{Mutex,Arc};

    let m = Mutex::new(5);
    {
        let mut num1 = m.lock().unwrap();
        *num1 = 6;
    }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let han = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(han);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("{}",*counter.lock().unwrap());
    
}
