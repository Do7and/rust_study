use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    pub query:String,
    pub filename: String,
    pub case_sensitive:bool,
}
impl Config {
    pub fn new(args:&[String])-> Result<Config,&'static str>{
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename  = args[2].clone();
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        if let Some(a) = args.get(3){
            println!("Some={}",a);
            case_sensitive = "1".eq(a);
        }
        println!("cs={}",case_sensitive);
        Ok(Config { query,filename, case_sensitive}) 
    }    
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive{
        search_insensitive(&config.query, &contents)
    }else{
        search_sensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}",line);
    }
    
    Ok(())
}

pub fn search_sensitive<'a>(query:&str,contents: &'a str)-> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines(){
        if line.contains(&query){
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive<'a>(query:&str,contents: &'a str)-> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = vec![];
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search_sensitive(query,contents));
    }
    #[test]
    fn case_sensitive(){
        let query: &str = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],search_sensitive(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query: &str = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_insensitive(query,contents));
    }
}

