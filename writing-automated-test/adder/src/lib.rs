#[derive(Debug)]//to print the struct and its values

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
 struct Rectangle{
     width:u32,
     height:u32,
 }  
 impl Rectangle {
    fn can_hold(&self,other:&Rectangle) -> bool{
        self.width > other.width && self.height > other.height

    }
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_Can_Hold_Smaller(){
        let larger =Rectangle{
        width:8,
        height:7,
    };
    
    let smaller =Rectangle{
        width:5,
        height:1,
    };  
    assert!(larger.can_hold(&smaller));//assert is a macro that returns true for test if the condition is true
    }}
    


