fn main(){
    // println!("Hello, world!");
     let four: IpAddrkind=IpAddrkind::v4(127,0,0,1);
    }
//Enums are  building blocks of Rust  and are really powerful 
//it allows to define a type by enumerating its possible variants

//ip address can be either v4 or v6 

enum IpAddrkind{
    v4(u8,u8,u8,u8),
    v6(),
}
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
//all varients are grouped under one type
 impl Message{
    fn sone_fun(){
        print!("lets finish!")
    }
 }