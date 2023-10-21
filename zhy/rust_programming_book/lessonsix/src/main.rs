fn main() {
    enum IpAddr {
        //枚举可以方便的为不同型定义一个父型
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    //在对 Option<T> 进行 T 的运算之前必须将其转换为 T 。通常这能帮助我们捕获空值最常见的问题之一：假
    //设某值不为空但实际上为空的情况。

    //Option相关方法

    //as_mut()
    //将可变的option转为option里的内容可变
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }
    assert_eq!(x, Some(42));

    //unwrap_or(self,default:T)
    //安全的提取some里的值
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    //match
    //每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值


    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),//（）为unit，啥也不会发生，_为通配符，相当于else
        }
        
    if let Some(3) = some_u8_value {
        println!("three");
    }

    //match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。
    //换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

}
