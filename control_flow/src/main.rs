fn main() {
    //Basic if statement
    // let number = 3;
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    //Will not work, type mismatch
    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    //Will work
    // let number = 3;
    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    //if chains
    // let number = 6;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    //Using a condition in a let statement
    // let condition = true;
    // let number = if condition {
    //     5
    // } else {
    //     6
    // };
    // println!("The value of number is: {}", number);

    //Will not work, type mismatch for assignment
    // let condition = true;
    // let number = if condition {
    //     5
    // } else {
    //     "six"
    // };
    // println!("The value of number is: {}", number);

    //Will loop forever
    // loop {
    //     println!("again!");
    // }

    //While loops
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number = number - 1;
    // }
    // println!("LIFTOFF!!!");

    //While looping over array index
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index = index + 1;
    // }

    //For loops
    // let a = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    //For loop range
    // for number in (1..4).rev() {
    // println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");
}
