fn main() {

let a: &[i32]=[1,2,3,4,5,6,7,8,9,10];
let mut v : Vec<i32> = Vec::new();//new fucntion on vector type so itll create the vector of type i32
v.push(5);
v.push(6);
let v2 : Vec <i32> =    vec![1,2,3,4,5,6,7,8,9,10];
//this vec! is a macro that will create a vector of any type
//macro is similar to function but with ! at the end
//diffrence is that macro takes variable number of parameters
let third:&i32=&v2[2];
println!("The third element is {}",third);//0 1 2
//let doesnot allow to change the value of the variable
matc v.get(2){
    Some(third)=>println("the third element is {}", third),
    None=>println("there is no third element"),
}
//printing them 
for i :&i32 in &mut v{
    println!("{}",i);

}

//could be used to solve index out of bound error gracefully without crashing the program

//stored in heap
//Strings are stored as a collection of UTF-8 encoded bytes


}

