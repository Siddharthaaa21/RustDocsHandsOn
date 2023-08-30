// fn main() {
//     let width = 20;
//     let height = 30;

//     println!(" the area is {} of the rectangle!", area(width, height));
// }

// fn area(w:u32,h:u32)->u32{
//     w*h
//     //return statement has no ; at the end

// }
/* Refactoring using tuples! */
//  fn main(){
//     let rec=(20.30);
//     println!(" the area is {} of the rectangle!", area(rec));
//  }
//  fn area (dimensions: (u32,u32))->u32{
//     dimensions.0*dimensions.1   
//  }
/*Rewfactoring with strucks! */
 
 struct Rectangle {
    width:u32,
    height:u32,

 }
 fn main(){
    let rec = Rectangle{
        width:32,
        height:22,
    };
    println!("the area of the rectangle is {}",area(&rec));

 }
 fn area(rectangle:&Rectangle) -> u32{
    rectangle.width * rectangle.height
 }


 //for running it on the main itself well have to use "#[derive(Debug)]" as an import and in print well have to use "{:?}" they are called debug annotations {:?} is for debug and {:#?} is for pretty debug
 /*derive debug is used to print the struct in a debug format*/

 /* dbg!(&rec) ==>> give the output of the struct with its balue declared! */