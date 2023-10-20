fn main() {
    //1.函数直接传入不可copy类型的数据，相当于东西被人抢走了，在函数里被绑票噶了
    //let s = String::from("Helloa");
    //do_sth(s);
    //println!("Here the string used : {}!",s); //此处会报错，s已经被do_sth绑走了
    //
    //2.像字符串String,不能制造备份
    //let s1 = String::from("hello");
    //let s2 = s1;  //这里会把s1失效掉，因为s1被move到s2了
    //println!("{}, world!", s1);//报错，s1已经噶了
    //let x = 5;
    //let y = x; //合法,因为int copy并不特别耗费内存
    //
    //3.返回值也可以转移作用域，所以在噶之前可以靠返回留一命
    //
    //4.如何防止被函数吃掉呢？借用&。类似腾讯在线word的编辑与查看
    //不可变引用类似于一个东西只有一份，只给你read权力不给改的权力
    //可变引用类似于借走给你改，但是可变引用只能有一个，同时不能在有人不可变引用的时候进行可变引用
    //let mut s = String::from("hello");
    //let r1 = &mut s;
    //let r2 = &mut s;//报错，不合法的二次可变引用
    //
    //5.字符串 slice range 的索引必须位于有效的 UTF-8
    //  字符边界内，如果尝试从一个多字节字符的中间位置创建字
    //  符串 slice，则程序将会因错误而退出
    //
    //
    //
}
fn calculate_length(s: &String) -> usize {
    //需要字符串参数的函数不鼓励&String,因为&str可以做到相同的效果，传入&str或String的切片如s[::](类型为&str)
    s.len()
}

fn do_sth(some_str:String){
    println!("The string used here : {}",some_str);
}

//fn dangle() -> &String {
//    //悬垂引用，在被噶之前把自己的身份证号留下来是不行滴
//    let s = String::from("hello");
//    &s
//
//}

fn first_word(s: &String) -> &str {
    //计算句子中第一个单词结束的索引
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

