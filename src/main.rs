use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;

fn main() {
    //reads any cli arguments and collects values into vector
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    for line in search(&config.query, &contents)  {
        println!("{line}");
    }
    
    Ok(())//ok case --> void
    
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 2 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok( Config{query, file_path}) //ok case /return type
    }
}
