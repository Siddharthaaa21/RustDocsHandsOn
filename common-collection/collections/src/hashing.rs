use std::collection::HashMap;

fn main(){
     let blue:String = String::from("blue");
     let yellow:String = String::from("yellow");

     let mut scores:HashMap<String,i32> = HashMap::new();
     scores.insert(blue,10);
        scores.insert(yellow,50);
        let team_name = String::from("blue");
        let score : Option<&i32> = scores.get(&team_name);
        //this will return the value of the key blue
        /* 
         */
      fot (key,value) in &scores{
          println!("{}:{}",key,value);
      }
      /* 
         this will print the key value pair
      
       */
scores.entry(String::from("yellow")).or_insert(50);
//this will insert the value 50 if the key yellow is not present
//if the key is present then it will return the value of the key
//this will not insert the value if the key is present
scores.entry(String::from("blue")).or_insert(50);

scores.entry(String::From("purple")).or_insert(50);
//this will insert the value 50 if the key purple is not present
let mut map: HashMap<String, Vec<i32>> = HashMap::new();
map.entry(String::from("Yellow")).or_insert(vec![50]);
map.entry(String::from("Yellow")).or_insert(vec![50, 25]);
//this will insert the value 50 if the key yellow is not present
//if the key is present then it will return the value of the key
    }