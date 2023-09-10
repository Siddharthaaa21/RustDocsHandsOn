// fn main() {
//     panic!("crash and burn");
//     println!("Hello, world!");//will not be executed 
    
// }
//panic! macro is used to cause a panic and stop execution of the program


/* seeing a backtrace  */
fn main(){
    a();
}
fn  a(){
    b();
}
fn b(){
    c(22);
    panic!("crash and burn");/* this will not be executed */
}
fn c(x:i32){
    println!("{}",&x);
    panic!("crash and burn");/*will stop the b panic to run */
}
/* to check  where the error actually is or where it is causing error we'll run the cargo run cmd with addition 
RUST_BACKTRACE=1 cargo run in linux
$env:RUST_BACKTRACE=1
cargo run
in windows
 */