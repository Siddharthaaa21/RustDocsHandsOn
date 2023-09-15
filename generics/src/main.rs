use std::cmp::PartialOrd;
fn main() {
    println!("Hello, world!");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result2=largest(&char_list);
    println!("The largest char is {}", result2);

}
fn get_largest<T:PartialOrd + Copy> (number_list:&Vec<T>)->T{
    let mut largest = number_list[0];
    for number in number_list{
        if number > &largest{
            largest = number;
        }
        largest 
    
    }
    
}
///similiarly we can use in enums(result and optional) structs as well 
/*

 struct Point<T,U>{
    x:T,
    y:U,
 }//generics specific here are not tired with generics specified in the imp block below
  imp <U> Point<U>{
    fn x(&self)-> &U{
        &self.x
    }
  }
  imp  Point<f64>{
    fn y(&self)-> &U{
        &self.y
    }
  }
  fn main (){
    let p=Point{x:5,y:10};
         p=Point{x:5.0,y:10.0};

  }

*/ 
//mix up function
impl <T,U> Point <T,U>{
    fn mixup<V,W> (self,other:Point<V,W>)->Point<T,W>{
        Point{
            s:self.s,
            p:self.p,
        }
    }
}
 