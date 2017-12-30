//VECTOR BASICS
// fn main() { 
//   let mut v = vec![1, 2, 3, 4, 5];
//
//   v.push(6);
//   v.push(7);
//   v.push(8);
//
//   //let third: &i32 = &v[2]; //Error if out of range of vector
//   //let third: Option<&i32> = v.get(2); //returns none if out of range of vector
//
//   //making changes with mutable borrows in a for loop
//   for i in &mut v {
//     *i += 50; //dereference i before change is made
// }
//
//   println!("{:?}",v )
// }


//ENUMS AND VECTORS
// enum SpreadsheetCell {
//   Int(i32),
//   Float(f64),
//   Text(String),
// }
//
// fn main() { 
//   //Vectors can store multiple data types using enums
//   let row = vec![
//     SpreadsheetCell::Int(3),
//     SpreadsheetCell::Text(String::from("Blue")),
//     SpreadsheetCell::Float(10.12),
//   ];
//
// }


//STRING FORMATTING AND ITERATING
// fn main(){
//   let s1 = String::from("tic");
//   let s2 = String::from("tac");
//   let s3 = String::from("toe");
//
//   let s = format!("{}-{}-{}", s1, s2, s3);
//
//   println!("{}", s);
//
//   //iterate over individual chars
//   for c in s.chars() { 
//     println!("{}", c);
//   }
// }


//HASH MAP DEFINITION
// use std::collections::HashMap;
//
// fn main() {
//   let mut scores = HashMap::new();
//
//   scores.insert(String::from("Blue"), 10);
//   scores.insert(String::from("Yellow"), 50);
// }


//HASH MAP ZIPPING
// use std::collections::HashMap;
//
// fn main() {
//   let teams  = vec![String::from("Blue"), String::from("Yellow")];
//   let initial_scores = vec![10, 50];
//
//   //creates hash map using teams as keys to their scores
//   let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); 
// }


//HASH MAP ACCESS
// use std::collections::HashMap;
//
// fn main() {
//   let mut scores = HashMap::new();
//
//   scores.insert(String::from("Blue"), 10);
//   scores.insert(String::from("Yellow"), 50);
//
//   //accessing with string
//   let team_name = String::from("Blue");
//   let score = scores.get(&team_name);
//
//   //accessing all keys/values with a for loop
//   for (key, value) in &scores {
//     println!("{}: {}", key, value);
//   }
// }


//HASH MAP VALUE CHANGES
// use std::collections::HashMap;

// fn main() {
//   //OVERWRITING
//   // let mut scores = HashMap::new();
//   //
//   // scores.insert(String::from("Blue"), 10);
//   // scores.insert(String::from("Blue"), 25); //overwrites previous value associated with blue
//   //
//   // println!("{:?}", scores);
//   ///////////////////////////////////////
//   //CHECKING IF EXISTS BEFORE OVERWRITING
//   // let mut scores = HashMap::new();
//   // scores.insert(String::from("Blue"), 10);
//   //
//   // scores.entry(String::from("Yellow")).or_insert(50);
//   // scores.entry(String::from("Blue")).or_insert(50);
//   //  
//   // println!("{:?}", scores);
//   ///////////////////////////////////////
//   //USING MUTABLE REFERENCE TO UPDATE VALUES
//   // let text = "hello world wonderful world";
//   //
//   // let mut map = HashMap::new();
//   //
//   // for word in text.split_whitespace() {
//   //     let count = map.entry(word).or_insert(0); //or_insert returns a mutable reference to the value
//   //     *count += 1; //mutable reference is updated
//   // }
//   //
//   // //prints how many times words occured
//   // println!("{:?}", map);

// }