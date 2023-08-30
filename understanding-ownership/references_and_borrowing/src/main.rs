fn main() {
let s =String::from("hello");
let len= calc_len(&s);
//passing the reference
println!("The length of {} is {}",s,len);
}
fn calc_len(s: &String) -> usize {
    s.len()
}


//2 Mutable References
// change(&mut s); ((push_str is a method of String type which appends a literal to the string for eg push_str(" world") will append world to the string))


//3 Scope of References
// let r1=&s; immutable references
// let r2=&s; immutable references
// let r3=&mut s; mutable references


//4 Dangling References
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// fn main() {
//     let reference_to_nothing = dangle();
// }


//5 Slices
// they are references to a contiguous sequence of elements in a collection rather than the whole collection

//let a = [1, 2, 3, 4, 5];
//let slice = &a[1..3]; // Creating a slice that references [2, 3]

//String slices:
// let s = String::from("hello world"); is also possible