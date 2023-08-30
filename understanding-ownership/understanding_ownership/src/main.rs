
//transfering ownership
//let s = String::from("hello"); s  is the owner of hello now
//let s2 = s; // s is no longer valid an dhte ownership is transferred to s2

//cloning data:
//let s1 = String::from("hello");
// let s2= s1.clone(); now s1 is still valid wiht s2 having the same value as s1

//ownership and functions


// fn main() {
//     println!("Hello, world!");
//     let s1 = String::from("hello");
//     takes_ownership(s1); //s value is moved into the function and is no longer valid

// }
// fn takes_ownership(s1: String){

//     let s=s1;//s1 now is no longer valid

// }

//return values and scope
fn gives_ownershit()->String{
let s1 = String::from("hello");
    s1 
}

fn main (){
    let s2 = gives_ownershit();
    println!("{}",s2);
}
/* but for interger this contradicts since they are stored on the stack while the string is stored on the heap
 */