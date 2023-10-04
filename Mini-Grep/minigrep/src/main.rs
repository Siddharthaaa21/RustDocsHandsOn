use std::env;//import a module from standard library to use env::args() method to get command line arguments
use std::fs;//import a module from standard library to use fs::read_to_string() method to read the contents of a file  
use std::process;
use std::error::Error;
//import a module from standard library to use process::exit() method to exit the program
fn main(){
    let args : Vec<String> = env::args().collect();// collect() method to turn iterator into a collection and store it into vector of args
  println!("{:?}", args );//{:?} is used to print the vector of string

//store query and call it now
let config=Config::new(&args).unwrap_or_else(|err|{
  println!("Problem parsing arguments: {}", err);
  process::exit(1);
});

println!("Searching for {}", config.query);
println!("in file {}", config.filename);
if let Err(e) = run(config){
  println!("Application error: {}",e);
  process::exit(1);
}



}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  println!("With text:\n{}", contents);
  Ok(())
}
  

struct Config{
  query:String,
  filename:String,
}
impl Config{
  fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    Ok(Config { query, filename })
  }
}
  
  

