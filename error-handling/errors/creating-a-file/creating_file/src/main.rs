// use std::fs::File;
//  use std::io::ErrorKind;
// // importing the file shart from  module from the standard library 
// fn main(){
//     let f = File::open("hello.txt");
// let f = match f {
//     Ok(file)=>file,
//     Err(error)=>match error.kind(){
//         ErrorKind::NotFound=>match File::create("helle.txt"){
//             Ok(fc)=>fc,
//             Err(e)=> panic!("Problem creating the file: {:?}",e),
//         },
//         other_error=>{
//             panic!("Problem opening the file: {:?}",other_error)

//         }
//     }
// };
// }
// /* simplifying the code using closures */
// /* fn main(){
//     let f=File::opem("hello.txt").expect(""failed to open hello.txt");

// }
// /* to further simplifying this code!!
// use std::fs::File;
// use std::io;
// use st::io::Read;

// fn read_username_from_file()-> Result<String,io::error>{
//     let mut s=String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }
// fn main(){


// }
// fn read_username_form_file()->Reult<String,io::Error>{
//     fs::read_to_string("hello.txt")//import ==>>use std::fs::{self,File};
// }
