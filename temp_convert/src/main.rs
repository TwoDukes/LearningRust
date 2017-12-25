use std::io;

fn main() {
    println!("convert Temperatures");

    //CorF = celcius or fareneheit
    let mut corf = String::new();

    println!("Press 1 to convert to celcius and 0 for farenheit");

    io::stdin().read_line(&mut corf).expect("Error");

    let corf: i32 = corf.trim().parse().expect("Please enter a number");

    println!("Enter a temperature");

    let mut temp = String::new();
    
    io::stdin().read_line(&mut temp).expect("Error");

    let mut temp: i32 = temp.trim().parse().expect("Please enter a number");

    let temp_string;
    //The temp conversion outputs for both corf options
    //See if this can be shortened
    if corf == 1 {
        temp_string = String::from("Celcius");
        temp = (temp - 32) * 5/9;
       } else {
        temp_string = String::from("Farenheit");
        temp = (temp * 9/5) + 32;
       };
      
    println!("{} degrees {}", temp, temp_string);
}
