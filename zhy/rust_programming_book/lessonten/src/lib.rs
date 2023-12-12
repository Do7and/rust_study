pub trait Summarizable{
    fn author_summary(&self) -> String;

    fn summary(&self) -> String{
        format!("Read more from {}...",self.author_summary())
    }
}
//声明一个trait定义，并提供实现这个trait所要实现的方法
//也可以像这样通过给trait中的方法一个默认实现，从而使其不再必须重载，此时必须实现的就是author_summary方法了

pub struct NewsArticles {
    pub headline:String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticles {
    fn author_summary(&self) -> String{
        "".to_string()
    }
    fn summary(&self) -> String{
        format!("{}, by {} ({})",self.headline,self.author,self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String{
        "".to_string()

    }
    fn summary(&self) -> String {
        format!("{}:{}",self.username ,self.content)
    }
}

pub fn notify<T: Summarizable>(item: T){
    //trait bounds 给泛型上限制，必须实现了Summarizable的泛型T才能传进来
    //可以用＋号连接多个限制
    println!("Breaking news! {}",item.summary());
}
//拥有多个泛型参数并每一个都有trait bounds时，可以用where从句

use std::fmt::Display;
use std::fmt::Debug;

fn some_function<T,U>(t:T,u:U) ->i32
where T: Display + Clone,
        U: Clone+ Debug
        {
            0
        }
//还可以区别对待，即给带着VIP卡的老爷开个贵宾通道

struct Welcome<T>{
    x:T,
    y:T,
}
impl<T> Welcome<T>{
    fn normalroom(x:T,y:T) -> Self{
        Self{x,
            y,
        }
    }
}
impl<T:Display + PartialOrd> Welcome<T>{
    fn viproom(&self){
        println!("This is VIP room!!");
    }
}