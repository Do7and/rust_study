
fn main() {
    //简化的匿名结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    //最朴实的结构体创建法
    let myuser = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    //加上debug注解修饰结构体后，可以使用
    println!("myuser is {:?}", myuser);
    println!("myuser is {:#?}", myuser);

    //结构体的方法:方法语法（method syntax）
    println!("myuser ue is {}", myuser.getue());

    //在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（ &self ），做出修改（ &mut self ）
    //或者是获取所有权（ self ）
    //故我们可以直接myuser.而非&myuser.

    //关联函数（associated functions）
    //没有self参数却写在impl块里的函数
    //String::from就是一个
    //可以用来制作构造函数
    //使用结构体名和 :: 语法来调用这个关联函数
    //如let sq = Rectangle::square(3);

}
#[derive(Debug)]//debug注解
struct User {
    //可以使结构体储存被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes）
    //email: &str,
    //防止出现悬垂指针
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
impl User{
    fn getue(&self) -> String{
        format!("{} {}",&self.username,&self.email)
    }
}

fn build_user(email: String, username: String) -> User {
    //同名赋值可以省略
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
    
