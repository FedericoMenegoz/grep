// we need to use std::env::args to be able to retrieve the argument passed
// to the command line cargo run -- <args>
// std::fs needed to handle files
use std::{env, fs, process, error::Error};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

/*
The return type Result<(), Box<dyn Error> lets us customize the error handling,
if everything goes well we will return the unit type () (ok case)
Box <dyn Error> is a trait object which means that the function will return a type
that implement the Error trait. This will permit more flexibility in addressing different
kind of error.
dyn = dynamic
*/
fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file!");

    println!("With text:\n{contents}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
