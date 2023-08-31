#[derive(Debug)]
fn main() {

    let rec=rectangle{
        width: 30,
        height: 50,
    };  
    println!("The area of the rectangle is {} square pixels.", rec.area());
}
struct rectangle{
    width: u32,
    height: u32,    
}
impl rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
}
