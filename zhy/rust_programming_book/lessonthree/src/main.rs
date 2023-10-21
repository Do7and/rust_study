use std::io;
fn main() {
    println!("1.Convension between Celsius and Fahrenheit!");
    println!("Please input 52 C to indicate 52 Celcius, or 130 F to indicate 130 Fahrenheit.");
    
    let mut inputpattern = String::new();
    io::stdin().read_line(&mut inputpattern)
        .expect("Failed to read line");
    
    let mut iter = inputpattern.split_whitespace();
    let temval:&str = match iter.next(){
        Some(text) =>text,
        None => "NO",
    };
    let temtype:&str = match iter.next(){
        Some(text) =>text,
        None => "NO",
    };
    println!("tempval{}\ntemptype{}",temval,temtype);

    let temval: f32 = temval.trim().parse().expect("Temperature invalid! ");
    if temtype == "C"{
    println!("Converted Temperature is {:+.2}F",(temval *9.0 /5.0)+32.0);
    }else if temtype == "F"{
    println!("Converted Temperature is {:+.2}C",(temval-32.0)*5.0/9.0);
    }else{
    println!("No valid input!");
    }

    println!("2.N-order Fibonacci sequence");
    println!("Please input n(n>2).");
    
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a positive interger");
    let mut fibonacci_array : Vec<i32> = Vec::with_capacity(n);
    fibonacci_array.push(1);
    fibonacci_array.push(1);
    println!("The n-order fibonacci sequence starts.");
    println!("1");
    println!("1");
    let mut poi = 2;
    while poi < n{
        fibonacci_array.push(fibonacci_array[poi-2] + fibonacci_array[poi-1]);
        println!("{}",fibonacci_array[poi]);
        poi = poi + 1;
    }
    println!("The n-order fibonacci sequence ends.");


}   
