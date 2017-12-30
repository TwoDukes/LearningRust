pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}

//USE EXAMPLES
///////////1/////////////
//
// pub mod a {
//     pub mod series {
//         pub mod of {
//             pub fn nested_modules() {}
//         }
//     }
// }
//
// use a::series::of;
//
// fn main() {
//     of::nested_modules();
// }
////////////2/////////////
//
// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }
//
// use TrafficLight::{Red, Yellow};
//
// fn main() {
//     let red = Red;
//     let yellow = Yellow;
//     let green = TrafficLight::Green;
// }
///////////3////////////////
//
// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// use TrafficLight::*;

// fn main() {
//     let red = Red;
//     let yellow = Yellow;
//     let green = Green;
// }
