fn main() {
    println!("Hello, world!");

    //指定生命周期的结构体，使得结构体的命跟里面特定变量的一样
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
    .next()
    .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
        

}
//枚举也可以拥有多个泛型类型
//当需要在函数体中使用一个参数时，必须在函数签名中声明这个参数以便编译器能知道函数体中这个名称的意义
//签名中可以加入trait bound来限制T
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
    if item > largest {
    largest = item;
    }
    }
    largest
    }
    

//Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个将泛型代码转变为实际放入
//的具体类型的特定代码的过程。

//生命周期
//编译器的这一部分叫做 借用检查器（borrow checker），它比较作用域来确保所有的借用都是有效的。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//如果函数内部产生了有所有权的东西
//在这种情况，最好的解决方案是返回一个有所有权的数据类型而不是一个引用

//生命周期省略规则（lifetime elision rules）
//1. 每一个是引用的参数都有它自己的生命周期参数。
//2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：
//3. 如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self ，那么 self 的生命周期
//被赋给所有输出生命周期参数

//'static 生命周期存活于整个程序期间。

//所有的字符串字面值都拥有'static 生命周期