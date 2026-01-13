use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter in the temperature:");
    io::stdin().read_line(&mut input1).expect("Not a valid string"); //Input the temperature
    let x:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Is it Celsius or Fahrenheit (C or F)?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let answer = input2.trim();

    // If Statement for Conversion
    if answer == "C" || answer == "c" { 
        let formula = (x*(9.0/5.0)) + 32.0; //F = C(9 ⁄ 5) + 32.
        println!("Converted to {:?} Fahrenheit",formula);
    }
    else if answer == "F" && answer == "f" {
        let formula = (x - 32.0) * (5.0/9.0); //C = (F − 32) × 5 ⁄ 9
        println!("Converted to {:?} Celsius",formula);
    }
    else{
        println!("Error!")
    }

}
