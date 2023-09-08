fn main(){
    //in rust there are no null values 
    //instead we have option enum 
    
    let five : Option<i32> =Some(5);
    let six:Option<i32>=plus_one(five); 
    let none:Option<i32>=plus_one(None); 

}
fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        None=>None,
        Some(i)=>Some(i+1),
    }
}
///match opertion matches with all the conditions!
// let Some_number : Option<i32>=Some(5);
// let Some_String : Option<i32>=Some("  ");
// let No_number : Option<i32>=None; 

//Optional type cannot be added to interger type
//to do this qw could use "unwrap_or(default:0);"
//for eg 
/*
let x:i8=5;
let y: Option<i8> = None;
let sum :i8 =x+y.unwrap_or(0);
 */