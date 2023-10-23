fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    //当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None
    let third: Option<&i32> = v.get(2);

    //无效引用报错
    //let first = &v[0];
    //v.push(6);
    //不可，因为&v在被人看着

    let mut v = vec![100, 32, 57];
    //遍历不改
    for i in &v {
        println!("{}", i);
    }
    //遍历改
    for i in &mut v {
        *i += 50;//解引用
    }

    //试下像String这种不可copy的会如何表现
    let mut vstr : Vec<String> = vec![String::from("Hello"),String::from("World")];
    let str1 = &vstr[0];
    println!("str1:{}",str1);
    println!("&vstr[0]:{}",&vstr[0]);
    let str2 = str1;
    println!("str2:{}",str2);
    println!("&vstr[0]:{}",&vstr[0]);
    //str1:Hello
    //&vstr[0]:Hello
    //str2:Hello
    //&vstr[0]:Hello

    let mut str3 = &mut vstr[0];
    str3.push_str("BAD");
    println!("str3:{}",str3);
    println!("&vstr[0]:{}",&vstr[0]);
    //str3:HelloBAD
    //&vstr[0]:HelloBAD


    //
    let strout = String::from("Live?");
    let mut vstrin : Vec<String> = vec![strout];
    //println!("strout:{}",strout);//所有权会被抢
    println!("&vstrin[0]:{}",&vstrin[0]);
    //字符串存储了 UTF-8 编码的文本



    //对于像 String 这样拥有所有权的值，其值将
    //被移动而哈希 map 会成为这些值的所有者，

    //检查某个特定的键是否有值，如果没有就插入一个值
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);

    //是找到一个键对应的值并根据旧的值更新它
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    //另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，其中每个元组包含一个键值对。
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

}
