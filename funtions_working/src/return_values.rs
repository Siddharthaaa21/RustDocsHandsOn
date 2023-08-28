fn main(){
    let x=return_valued_function(10);
    println!("The value returned from the function is {}",x);
}
fn return_valued_function(x:i32)->i32{
    x+10
}