use std::io;

fn main() {
    println!("convert Temperatures");

    let mut corf = String::new();

    println!("Press 0 for celius and 1 for farenheit");

    io::stdin().read_line(&mut corf).expect("Error");

    let corf: i32 = corf.trim().parse().expect("Please enter a number");

    println!("Enter a temperature");

    let mut temp = String::new();
    
    io::stdin().read_line(&mut temp).expect("Error");

    let mut temp: i32 = temp.trim().parse().expect("Please enter a number");

    temp = if corf == 1 {
        (temp - 32) * 5/9
       } else {
        (temp * 9/5) + 32
       };
      
    
    println!("{} degrees", temp);
}
