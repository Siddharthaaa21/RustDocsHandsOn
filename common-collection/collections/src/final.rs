//enums and vectors 
fn main(){
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(Strin),
    }
let row:Vec<SpreadsheetCell>=vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    ];
    match &row[1]{
        SpreadsheetCell::Int(i)=>println!("the value is {}",i),
        SpreadsheetCell::Float(f)=>println!("the value is(float) {}",f),
        SpreadsheetCell::Text(s)=>println!("the value is (String) {}",s),
        =>println!("the value is none"),
    }


}