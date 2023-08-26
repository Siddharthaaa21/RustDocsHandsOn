// fn main() {
//     println!("Hello, world!");
// }
use rand::Rng;
use std::cmp::Ordering;
use std::io;
//use to create odering type that we can compare to other values

//standard library for the std and io is the defalut one just like java it is  java.io
fn main() {
    println!("Hey! guess the number");
    loop {
        println!("enter your input!");
        let mut var = String::new();
        //creating a mutable variable that is currently bound to a new emplt instance of a String
        //mut for mutable
        //let is for taking variable like let a=10; (immutable)....and let mut a=; (mutable)
        io::stdin()
            //io for input output so basically
            .read_line(&mut var)
            //read_line is a method that takes whatever the user types into standard input and places that into a string is given to
            .expect("could read");
       
            let var: u32 = match var.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        println!("you guessed: {var}");
        // I specified the type of var as i32 when parsing it, so it matches the type of secret_number.

        // I specified the type of var as i32 when parsing it, so it matches the type of secret_number.
        let secret_number = rand::thread_rng().gen_range(14..=101);
        println!("the secreate number is {secret_number}");

        match var.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
        }
    }
}
