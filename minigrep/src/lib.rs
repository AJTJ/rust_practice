use std::error::Error;
use std::fs;

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
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  // returning a Result allows "new" to be handled more cleanly
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    // cloning to avoid having to deal with lifetimes and makes it rather straightforward
    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

//Box<dyn Error> means the function will return a type that implements the Error trait, but we don't have to specify what particular type the return value will be.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  for line in search(&config.query, &contents) {
    println!("{}", line);
  }
  Ok(())
}
