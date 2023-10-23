pub fn add(left: usize, right: usize) -> usize {
    left + right
}
//关于模块
//Rust 默认只知道 src/lib.rs 中的内容
//要么文件夹命名模块名，同时目录下放mod.rs代表自己
//要么就叶子节点直接用模块名命名rs

//如果一个叫做 foo 的模块没有子模块，应该将 foo 的声明放入叫做 foo.rs 的文件中。
//如果一个叫做 foo 的模块有子模块，应该将 foo 的声明放入叫做 foo/mod.rs 的文件中。

mod client;
mod network;
//引用外部模块，只需要在lib里按模块规则设置，并在main.rs里写 extern crate communicator;

//可见性
//公有会贯穿整个模块树，而私有只会当前模块里生效
//模块和函数各自都有公私属性

//use 关键字只将指定的模块引入作用域；它并不会将其子模块也引入。
//可以使用 * 语法，这称为 glob 运算符（glob operator）

//::client::connect(); 从根目录开始找
//super::client::connect(); 从上级目录找，相当于找同事要东西，client你同事
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
