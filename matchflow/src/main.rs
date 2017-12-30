// COIN MATCHING
// #[derive(Debug)] // So we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // ... etc
// }
//
// enum Coin {
//   Penny,
//   Nickle,
//   Dime,
//   Quarter(UsState)
// }
//
// fn main() {
//     println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)))
// }
//
// fn value_in_cents(coin: Coin) -> u32 {
//   match coin {
//       Coin::Penny => 1,
//       Coin::Nickle => 5,
//       Coin::Dime => 10,
//       Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//       },
//   }
// }

//OPTION MATCHING
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
//
// fn main() {
// 
//   let five = Some(5);
//   let six = plus_one(five);
//   let none = plus_one(None);
// }


//DEFAULT MATCH
// let some_u8_value = 0u8;
// match some_u8_value {
//     1 => println!("one"),
//     3 => println!("three"),
//     5 => println!("five"),
//     7 => println!("seven"),
//     _ => (), //the default
// }


// IF LET
//
// //THESE TWO ARE EQUIVALENT 
/////////////////////////////////////
// let some_u8_value = Some(0u8);
// match some_u8_value {
//     Some(3) => println!("three"),
//     _ => (),
// }
////////////////////////////////////////
// if let Some(3) = some_u8_value {
//     println!("three");
// }
////////////////////////////////////////
////////////////////////////////////////
//
// //THESE TWO ARE EQUIVALENT 
///////////////////////////////////////////////
// let mut count = 0;
// match coin {
//     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//     _ => count += 1,
// }
////////////////////////////////////////////////
/// let mut count = 0;
// if let Coin::Quarter(state) = coin {
//     println!("State quarter from {:?}!", state);
// } else {
//     count += 1;
// }
///////////////////////////////////////////////