use std::io;
fn main() {
    let a =[1,2,3,4,5,6,7,8,9,10];
    loop{
    println!("please enter the index of the array which you want to see");
    
     let mut index=String::new();
      io::stdin()
      .read_line(&mut index)
      .expect("could not read");
    let index:usize= match index.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    if index<a.len(){
        println!("index entered was {} and the element is {}",index,a[index]);
        break;
    }
    else{
        println!("index out of bound");
        continue;
    }
   }


}
