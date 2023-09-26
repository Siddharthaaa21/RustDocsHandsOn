use std::env;//import a module from standard library to use env::args() method to get command line arguments
use std::fs;//import a module from standard library to use fs::read_to_string() method to read the contents of a file  

fn main(){
    let args : Vec<String> = env::args().collect();// collect() method to turn iterator into a collection and store it into vector of args
  println!("{:?}", args );//{:?} is used to print the vector of string

//store query and call it now
let config=parse_config(&args);
println!("Searching for {}", config.query);
println!("in file {}", config.filename);
let content = fs::read_to_string(config.filename).expect("something went wrong");
println!("With text:\n{}", content);

}
struct Config{
  query:String,
  filename:String,
}

fn parse_config(args:&[String ]) -> Config{
  let query=args[1].clone();
  let filename=args[2].clone();
  Config{query,filename}

}