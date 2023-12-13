use List::{Cons,Nil};
use std::rc::{Rc,Weak};
use std::cell::RefCell;

// enum List {
//     Cons(i32,Box<List>),
//     Nil,
// }

// fn main() {
//     let list = Cons(1,
//     Box::new(Cons(2,
//     Box::new(Cons(3,
//         Box::new(Nil))))));
// }



//BOX 存在堆上



fn hello(name: &str){
    println!("hello ,{}",name);
}
//Deref 解引用trait  
//可以通过实现这个trait来将智能指针当作常规引用来处理
//比如box里装了个String去调用hello &Box<String> 被转为&String，再被转为&str，是个退化过程，退化完发现符合签名，于是就可以调用了
//deref 方法返回值的引用
//实现这个以后会自动帮你*，尤其是遇到解引用强制多态（deref coercions）
//Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。
//但是有约束，可变性不会增加
// 当 T: Deref<Target=U> 时从 &T 到 &U 。
// 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U 。
// 当 T: Deref<Target=U> 时从 &mut T 到 &U 。


//对于智能指针模式来说另一个重要的 trait 是 Drop
//Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop ，这会导致一个 double free 错误
//可以通过 std::mem::drop 提早丢弃值


//Rc
//有些情况单个值可能会有多个所有者
//为了启用多所有权，Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写。引用计数意味着
//记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。

// enum List {
//     Cons(i32,Rc<List>),
//     Nil,
// }

// fn main() {
//     let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3,Rc::clone(&a));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4,Rc::clone(&a));
//         println!("count after creating a = {}", Rc::strong_count(&a));
//     }
//     println!("count after creating a = {}", Rc::strong_count(&a));
// }


// Rc<T> 允许通过不可变引用来只读的在程序的多个部分共享数据
// TODO 这个List是如何知道我在创建他的，为什么改不了名


//RefCell
//内部可变性（Interior mutability）是 Rust 中的一个设计模式

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>,Rc<List>),
//     Nil,
// }


// fn main(){
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(Cons(Rc::clone(&value),Rc::new(Nil)));
//     let b = Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(10)),Rc::clone(&a));

//     *value.borrow_mut() += 10;
//     println!("a after = {:?}",a);
//     println!("b after = {:?}",b);
//     println!("c after = {:?}",c);
// }


//循环引用
#[derive(Debug)]
struct Node{
    value:i32,
    parent:RefCell<Weak<Node>>,
    children:RefCell<Vec<Rc<Node>>>,
}