
// 我们希望能够在程序的一个位置指定某些代码，并只在程序的某处实际需要结果的时候 执行 这些代码。这正是闭包的用武之地！
// 就是我希望一段程序有函数特征让我不用反复重复写，又不需要去提前初始化而是到了需要用的地方在进行初始化执行
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {//闭包的定义以一对竖线（ | ）开始，在竖线中指定闭包的参数；
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
    };
    if intensity < 25 {
    println!(
    "Today, do {} pushups!",
    expensive_closure(intensity)
    );
    println!(
    "Next, do {} situps!",
    expensive_closure(intensity)
    );
    } else {
    if random_number == 3 {
    println!("Take a break today! Remember to stay hydrated!");
    } else {
    println!(
    "Today, run for {} minutes!",
    expensive_closure(intensity)
    );
    }
    }
}
fn main() {
    println!("Hello, world!");
}
// 注意其定义并没有增加任何类型注解：如果尝试调用闭包两次，第一
// 次使用 String 类型作为参数而第二次使用 u32 ，则会得到一个错误