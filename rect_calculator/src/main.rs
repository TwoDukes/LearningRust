//NO STRUCT
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//       "The area of the rectangle is {} square pixels.",
//       area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32{
//   width * height
// }


//WITH TUPLES
// fn main() {
//
//     let rect1 = (30, 50);
//
//     println!(
//       "The area of the rectangle is {} square pixels.",
//       area(rect1)
//     );
// }
//
// fn area(dimension: (u32, u32)) -> u32{
//   dimension.0 * dimension.1
// }


//WITH STRUCTS
//output struct debug tag
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

fn main() {

    let rect1 = Rectangle{width: 30, height: 50};

    println!("rect1 is {:#?}", rect1);

    println!(
      "The area of the rectangle is {} square pixels.",
      area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32{
  rectangle.width * rectangle.height
}