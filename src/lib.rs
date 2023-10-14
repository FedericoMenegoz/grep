// we need to use std::env::args to be able to retrieve the argument passed
// to the command line cargo run -- <args>
// std::fs needed to handle files
use std::{fs, error::Error, env};

/*
The return type Result<(), Box<dyn Error> lets us customize the error handling,
if everything goes well we will return the unit type () (ok case)
Box <dyn Error> is a trait object which means that the function will return a type
that implement the Error trait. This will permit more flexibility in addressing different
kind of error.
dyn = dynamic
*/
pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        // @todo find a better solution to .clone() ?
        let query = args[1].clone();
        let file_path = args[2].clone();

        // this will check the presence of the environment var
        // @todo set the environment var through an args
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}
pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str> {
    contents.lines()
        .filter(|line| (*line).contains(query))
        .collect()
}
// @todo to_lowercase not accurate
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| (*line).to_lowercase().contains(&query.to_lowercase()))
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}