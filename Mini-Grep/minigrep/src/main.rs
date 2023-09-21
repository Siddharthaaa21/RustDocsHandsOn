use std::env;//import a module from standard library to use env::args() method to get command line arguments
use std::fs;//import a module from standard library to use fs::read_to_string() method to read the contents of a file  

fn main(){
    let args : Vec<String> = env::args().collect();// collect() method to turn iterator into a collection and store it into vector of args
  println!("{:?}", args );//{:?} is used to print the vector of string

//store query and call it now
let query=&args[1];
let filename=&args[2];
println!("Searching for {}", query);
println!("in file {}", filename);
let content = fs::read_to_string(filename).expect("something went wrong");
println!("With text:\n{}", content);

}