// use std::env;//import a module from standard library to use env::args() method to get command line arguments
use std::fs;//import a module from standard library to use fs::read_to_string() method to read the contents of a file  
// use std::process;
use std::error::Error;
 pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

  let contents = fs::read_to_string(config.filename)?;
for line in search_for(&config.query,&contents){
println!("{}", line);
}
  Ok(())
}


  

 pub struct Config{
   pub query:String,
  pub filename:String,
}
impl Config{
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    Ok(Config { query, filename })
  }
}
pub fn search_for<'a>(query:&str, contents:&'a str)-> Vec<& 'a str>{//have to be specified in these 
  let mut results=Vec::new();

  for line in contents.lines(){
    if line.contains(query){
      results.push(line);
    }
  }
  results
//whenevr we are returning an vector then we have to tie up with lifetime 
}
#[cfg(test)]
mod tests{
  use super::*;
  //importing everything form the parent mod
  #[test]
  fn one_result(){
    let query="duct";
    let contents= "\
    Rust :
    safe,fasr,productive.
    pick threee.";  
    assert_eq!(vec!["safe, fast, productive"],search_for(query,contents));
    
  }
}

  
