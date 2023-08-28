
// fn main() {
//     another_function();
//     println!("hello inside the main ")
// }
// fn another_function() {
//     println!("Hello, world! inside the funtion");
// }


//for char type ": char" instead ":i32"
use std::io;
fn main(){
    println!("hello world");
    println!("Enter the value you wan to display vai funtion calling!");
    let mut x=String::new();
    io::stdin()
    .read_line(&mut x)
    .expect("Failed to read line");

    
    match x.trim().parse::<i32>() {
        Ok(num) => 
        {
            another_function(num);
            }
        Err(_) => {
            println!("inavalid input");
            }
    };

}
fn another_function(x:i32){
    println!("The value you entered is {}",x);
    println!(" and the incremented on is {}",x+10);
    
}
// fn main(){
//     let x=return_valued_function(10);
//     println!("The value returned from the function is {}",x);
// }
// fn return_valued_function(x:i32)->i32{
//     x+10
// }