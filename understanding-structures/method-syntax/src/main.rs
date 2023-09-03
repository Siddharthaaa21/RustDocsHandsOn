#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,    
}
impl Rectangle{
    //impl is implementation fir the block of rectangle
    
    fn area(&self)->u32{
        //self is the reference in the methode.
        //&self is used to borrow the value from the struct
        self.width*self.height
    }
}

fn main() {

    let rec1 =Rectangle{
        width: 30,
        height: 50,
    };  
    let rec2=Rectangle{
        width: 33,
        height: 50,
    }; 
    let rec3=Rectangle{
        width: 36,
        height: 50,
    }; 
    println!("The area of the rectangle 1 is {} square pixels.", rec1.area());
    println!("The area of the rectangle 2 is {} square pixels.", rec2.area());
    println!("The area of the rectangle 3 is {} square pixels.", rec3.area());
}
//mutlioke impl blocks are allowed
//associated functions are allowed


//methodes with more parameters
