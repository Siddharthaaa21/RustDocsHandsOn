use std::fs::File;
 use std::io::ErrorKind;
// importing the file shart from  module from the standard library 
fn main(){
    let f = File::open("hello.txt");
let f = match f {
    Ok(file)=>file,
    Err(error)=>match error.kind(){
        ErrorKind::NotFound=>match File::create("helle.txt"){
            Ok(fc)=>fc,
            Err(e)=> panic!("Problem creating the file: {:?}",e),
        },
        other_error=>{
            panic!("Problem opening the file: {:?}",other_error)

        }
    }
};
}