/* is like option enum 
but instead of None, it has Err and Ok variants
*/
//  fn main(){
//     enum Result<T,E>{
//         Ok(T),
//         Err(E), 
//     }
//  }

use std::fs::File;
// importing the file shart from  module from the standard library 
fn main(){
    let f = File::open("hello.txt");//this will return a Result<T,E> in this case Result<File,error>
    //open method returns a Result<T,E>
    //if the file exists,open returns a value of type Result<T,E> with Ok value that is holding a file handle
    //let f:Result<File,error>=File::open("hello.txt");



    let f = match f{
        Ok(file)=>file,
        Err(error)=>panic!("Problem opening the file: {:?}",error),
    };
    }
