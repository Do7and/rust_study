use std::collections::HashMap;


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
    
    //官方文档的写法
    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }


    //另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，其中每个元组包含一个键值对。
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    //1.求vector的平均数/中位数/众数
    let v:Vec<i32>= vec![11,46,35,67,23,5,7,3,15,11,45,11,67,23,5,11,36,98];
    println!("AVG is {}",avgofvec(&v));
    println!("MEDIAN is {}",medofvec(&v));
    println!("MODE is {}",modofvec(&v));


    //2.转换字符串为Pig Latin 即第一个辅音被移到末尾并加-Xay，元音开头的则在末尾加-hay
    let teststr = String::from("As violence erupted in Israel over the weekend, Hamas militants infiltrated several communal settlements known as kibbutz near the border with Gaza");
    println!("Transfered String is {}",transfer2piglatin(&teststr));
    
    //3.实现一个文本接口，允许添加，获取所有的列表，或按字母序排序后的列表
    //不写了，思路是key为部门，val为vec的hashmap，解析输入空格分割后提起13位置填入
}

fn avgofvec(v: &Vec<i32>) -> f32 {
    let mut sum:i32 = 0;
    for val in v{
        sum = sum + val;
    }
    sum as f32 /v.len() as f32

}

fn medofvec(v: &Vec<i32>) -> f32 {
    let mut vmy = v.clone();
    let len = vmy.len();
    //怎么求呢，排序吧，排完序分奇偶
    if len%2 == 1{
        vmy.sort();
        vmy[len/2] as f32+0.0
    }else{
        vmy.sort();
        (vmy[len/2]as f32+vmy[len/2+1]as f32)/2.0 +0.0
    }
    
}
fn modofvec(v: &Vec<i32>) -> i32 {
    let mut numcount = HashMap::new();
    for num in v{
        let count = numcount.entry(num).or_insert(0 as i32);
        *count += 1;
    }
    //如何求val最大的key呢？
    let mut maxval = 1;
    let mut maxkey:i32 = -1;
    for (key, val) in numcount.iter() {
        if val > &maxval{
            maxval = *val;
            maxkey = **key;
        }
    }
    maxkey
}

fn transfer2piglatin(text: &str) -> String {
    fn isaeiou(ch:&str) -> bool{
        ["a","e","i","o","u"].iter().any(|&x| ch.to_ascii_lowercase() == x)
    }

    let mut res = String::new();
    for word in text.split_whitespace() {
        if isaeiou(&word[0..1]){
            res.push_str(&word);
            res.push_str("-hay");
        }else{
            res.push_str(&word[1..]);
            res.push_str("-");
            res.push_str(&word[0..1]);
            res.push_str("ay");
        }
        res.push_str(" ");
    }
    res
}