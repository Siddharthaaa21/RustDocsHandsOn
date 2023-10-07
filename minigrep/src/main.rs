use std::env;//import a module from standard library to use env::args() method to get command line arguments
use std::process;
use minigrep::Config;
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
if let Err(e) = minigrep::run(config){
  println!("Application error: {}",e);
  process::exit(1);
}



}
