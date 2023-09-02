#[derive(Debug)]
fn main() {

    let rec1 =rectangle{
        width: 30,
        height: 50,
    };  
    let rec2=rectangle{
        width: 33,
        height: 50,
    }; 
    let rec3=rectangle{
        width: 36,
        height: 50,
    }; 
    println!("The area of the rectangle 1 is {} square pixels.", rec1.area());
    println!("The area of the rectangle 2 is {} square pixels.", rec2.area());
    println!("The area of the rectangle 3 is {} square pixels.", rec3.area());
}
struct rectangle{
    width: u32,
    height: u32,    
}
impl rectangle{
    //impl is implementation fir the block of rectangle
    
    fn area(&self)->u32{
        ///self is the reference in the methode.
        //&self is used to borrow the value from the struct
        self.width*self.height
    }
}

//mutlioke impl blocks are allowed
//associated functions are allowed


//methodes with more parameters
